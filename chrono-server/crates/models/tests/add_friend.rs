use models::friend::friends::Friends;

#[tokio::test]
async fn test_add_friend() {
    if let Err(e) = (Friends {
        id: 1,
        pub_key: "".to_string(),
        peer_id: "".to_string(),
        avatar: "".to_string(),
        nickname: "".to_string(),
        is_group: 0,
        is_deleted: 0,
    }
    .insert()
    .await)
    {
        println!("{e}");
    }
}
