use serde_derive::{Deserialize, Serialize};
use tracing::info;


// Modify接口定义返回结构体
#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyResponse {
    pub code: u32,
    pub message: String,
}

// 定义Modify接口调用函数
pub async fn modify(base_url: &str, token: &str, order_number: &str, address: &str) -> Result<ModifyResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // address字符串因为要放在URL中，所以需要进行URL编码
    let address = url::form_urlencoded::byte_serialize(address.as_bytes()).collect::<String>();

    // 根据环境拼装接口地址
    let url = format!("{}/order/{}/modify?address={}", base_url, order_number, address);

    // 发送请求
    let resp = client.get(&url).header("Access-Token", token).send().await;
    info!("modify resp: {:?}", resp);

    
    // 处理返回结果
    return match resp {
        Ok(resp) => {
            let resp = resp.json::<ModifyResponse>().await;
            match resp {
                Ok(resp) => Ok(resp),
                Err(err) => Err(Box::new(err)),
            }
        }
        Err(err) => Err(Box::new(err)),
    };
}

#[cfg(test)]
mod tests {
    use std::panic;

    use mysql::prelude::Queryable;
    use rstest::rstest;
    use crate::{redis::UserInfo, db::MySQLConfig};
    
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    struct OrderInfo {
        pub order_nbr: String,
        pub address: String,
        pub order_status: u8,
        pub buyer_id: u32,
    }
    
    #[tracing_test::traced_test]
    #[rstest]
    #[case("s001", "Streat A, Pudong district, Shanghai", 1000, "Modify address success")]
    #[tokio::test]
    async fn test_modify_success(#[case] order_nbr: &str, #[case] address: &str, #[case] code: u32, #[case] message: &str) {
        info!("case: test_modify_success start, with params: order_nbr: {}, address: {}", order_nbr, address);
        // ---- 初始化测试数据 ----
        let env = super::super::get_env();
        info!("get env: {:?}", env);
        let mysql_conf = MySQLConfig::new(
            env.db_conf.host.clone(), 
            env.db_conf.port,
            env.db_conf.username.clone(), 
            env.db_conf.password.clone(), 
            "coffeedb".to_string(),
        );
        let token: &str = "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJsaXVkYW8iLCJleHAiOjE2NzA5ODkyNDV9.aegReDbly0asm4lC6aOCvn1gW26_cFGqmxqBeV-JI90";
        let user_info = UserInfo::new(
            1, 
            "liudao".to_string(), 
            "13511111111".to_string(), 
            0,
        );

        init(&env.redis_conf.url, &mysql_conf, token, user_info.clone(), order_nbr);

        // ---- 执行测试 ----
        // 发起请求
        let resp = modify(&env.url, token, order_nbr, address).await.unwrap();

        // ---- 验证结果 ----
        // 获取数据库数据
        let mut conn = mysql_conf.get_conn();
        let record = conn.exec_map(
            r"SELECT `orderNbr`, `orderStatus`, `buyerId`, `address` FROM `t_order` WHERE `orderNbr` = ?", 
            (order_nbr,), 
            |(order_nbr, order_status, buyer_id, address): (String, u8, u32, String)| {
                OrderInfo {
                    order_nbr,
                    order_status,
                    buyer_id,
                    address,
                }
            },
        ).unwrap();

        let result = panic::catch_unwind( || {
            //返回结果断言
            assert_eq!(resp.code, code);
            assert_eq!(resp.message, message);

            // 数据库记录断言
            assert_eq!(record.len(), 1);
            assert_eq!(record, vec![OrderInfo {
                order_nbr: order_nbr.to_string(),
                order_status: 0,
                buyer_id: user_info.account_id,
                address: address.to_string(),
            }]);
        });
        
        // ---- 清理测试数据 ----
        clean(&mysql_conf, order_nbr);

        if let Err(e) = result {
            panic::resume_unwind(e);
        }
       

    }

    fn init(redis_url: &str, db_conf: &MySQLConfig, token: &str, user_info: UserInfo, order_number: &str) {
        info!("init test case data");
        // 模拟用户登录
        init_redis(redis_url, &user_info, token);
        init_db(db_conf, user_info.account_id, order_number)
    }

    fn clean(db_conf: &MySQLConfig, order_number: &str) {
        info!("clean test case data");
        clean_db(db_conf, order_number);
    }

    fn init_redis(redis_url: &str, user_info: &UserInfo, token: &str) {
        info!("init redis data");
        let _ = user_info.clone().prepare_login_user(redis_url, token).expect("prepare login user failed");
    }

    fn init_db(db_conf: &MySQLConfig, account_id: u32, order_number: &str) {
        info!("init db data");
        let mut conn = db_conf.get_conn();
        let result = conn.exec_drop(
            r"INSERT INTO `t_order` (`orderNbr`, `orderStatus`, `buyerId`, `address`, `createTime`, `updateTime`) 
                VALUES (?, ?, ?, ?, now(), now());",
            (
                order_number,
                0,
                account_id,
                "Streat Z, Pudong district, Shanghai",),
        );
        assert!(result.is_ok(), "init db failed");
    }

    fn clean_db(db_conf: &MySQLConfig, order_number: &str) {
        info!("clean db data in `t_order`");
        let mut conn =db_conf.get_conn();
        let result = conn.exec_drop(
            r"DELETE FROM `t_order` WHERE `orderNbr` = ?;",
            (order_number,),
        );
        assert!(result.is_ok(), "clean db failed");
    }

}