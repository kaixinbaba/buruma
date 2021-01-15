#[macro_use]
extern crate log;

use std::thread;

use tokio::time::Duration;

use buruma::constants::CreateMode;
use buruma::protocol::req::ACL;

use buruma::{WatchedEvent, Watcher, ZKResult, ZooKeeper};
use futures_timer::Delay;

mod common;

#[tokio::test]
async fn basic() {
    let basic_path = "/buruma";
    let mut zk = ZooKeeper::new("127.0.0.1:2181", 6000).await.unwrap();
    Delay::new(Duration::from_secs(1)).await;
    // 以防万一先将该节点删除
    zk.delete(basic_path).await;

    // 增加节点
    let data = Some("I Love U".as_bytes());
    let path = zk
        .create(basic_path, data, ACL::world_acl(), CreateMode::Persistent)
        .await
        .unwrap();
    assert_eq!(path, basic_path);
    Delay::new(Duration::from_secs(1)).await;
    let stat = zk.set_data(basic_path, "buruma".as_bytes()).await.unwrap();
    info!("{:?}", stat);
    Delay::new(Duration::from_secs(1)).await;
    let get_data_result = zk.get_data_without_watcher(basic_path, None).await.unwrap();
    assert_eq!(
        "buruma".to_string(),
        String::from_utf8(get_data_result).unwrap()
    );
    Delay::new(Duration::from_secs(1)).await;
    // 删除节点
    zk.delete(basic_path).await.unwrap();
}

#[derive(Debug, Hash)]
struct WatcherDemo;

impl Watcher for WatcherDemo {
    fn process(&self, event: &WatchedEvent) -> ZKResult<()> {
        info!("{:?}", event);
        Ok(())
    }
}

#[tokio::test]
#[ignore]
async fn get_data() {
    let basic_path = "/xjj";
    let mut zk = ZooKeeper::new("127.0.0.1:2181", 60000).await.unwrap();

    let x = zk
        .get_data(basic_path, Some(WatcherDemo), None)
        .await
        .unwrap();
    info!("first {:?}", String::from_utf8(x));
    Delay::new(Duration::from_secs(10)).await;
    let x = zk.get_data_without_watcher(basic_path, None).await.unwrap();
    info!("from 1 {:?}", String::from_utf8(x));
}
