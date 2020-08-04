use demonstrate::demonstrate;

demonstrate! {
    #[async_attributes::test]
    async describe api -> Result<(), http_types::Error> {
        before {
            use orbit_db_http_client::Client;
            let client = Client::new(
                url::Url::parse("https://localhost:3000").unwrap()
            );
        }

        describe general {
            after { Ok(()) }

            test get_dbs { client.get_dbs().await?; }
            test get_identity { client.get_identity().await?; }
        }

        #[should_panic]
        describe err_scenarios -> () {
            before { let fake_db_name = "fake_db"; }

            test get_db {
                client.get_db(fake_db_name).await.unwrap();
            }
            test get_db_iterator {
                client
                    .get_db_iterator(fake_db_name, None)
                    .await
                    .unwrap();
            }
            test get_counter_value {
                client
                    .get_counter_value(fake_db_name)
                    .await
                    .unwrap();
            }
            test inc_counter_value {
                client
                    .inc_counter_value(fake_db_name, None)
                    .await
                    .unwrap();
            }
            test db_put {
                client.db_put(fake_db_name, &serde_json::json!(
                    { "_id": 1, "value": "test" }
                ))
                .await.unwrap();
            }
            context db_item {
                before {
                    let fake_db_item = "item";
                }

                test db_add {
                    client
                        .db_add(fake_db_name, fake_db_item)
                        .await
                        .unwrap();
                }
                test get_db_item {
                    client
                        .get_db_item(fake_db_name, fake_db_item)
                        .await
                        .unwrap();
                }
                test delete_db_item {
                    client
                        .delete_db_item(fake_db_name, fake_db_item)
                        .await
                        .unwrap();
                }
            }
        }

        // Inheritely tests `create_db()` and `delete_db()` for each type
        describe db_types {
            before {
                use orbit_db_http_client::DatabaseType;
                let real_db_name = String::from("real_db");
            }
            after {
                client.delete_db(&real_db_name).await?;
                Ok(())
            }

            describe feed {
                before {
                    client
                        .create_db(
                            &real_db_name,
                            DatabaseType::Feed,
                            None,
                            false
                        )
                        .await?;
                }

                test get_db { client.get_db(&real_db_name).await?; }
                test get_db_iterator {
                    client
                        .get_db_iterator(&real_db_name, None)
                        .await?;
                }
            }
            describe counter {
                before {
                    client
                        .create_db(
                            &real_db_name,
                            DatabaseType::Counter,
                            None,
                            false
                        )
                        .await?;
                }

                test get_counter_value {
                    assert_eq!(
                        client
                            .get_counter_value(&real_db_name)
                            .await?,
                        0
                    );
                }
                test inc_counter_value {
                    client
                        .inc_counter_value(&real_db_name, None)
                        .await?;
                }
            }
            describe eventlog {
                before {
                    client
                        .create_db(
                            &real_db_name,
                            DatabaseType::EventLog,
                            None,
                            false
                        )
                        .await?;
                }

                test db_add {
                    client.db_add(&real_db_name, "entry").await?;
                }
            }
            describe docstore {
                before {
                    client
                        .create_db(
                            &real_db_name,
                            DatabaseType::DocStore {
                                index_by: None
                            },
                            None,
                            false,
                        )
                        .await?;
                }

                describe db_item {
                    before {
                        let record = serde_json::json!(
                            { "_id": 1, "value": "test" }
                        );
                        client
                            .db_put(&real_db_name, &record)
                            .await?;
                    }
                    after {
                        client
                            .delete_db_item(&real_db_name, "1")
                            .await?;
                    }

                    test get_db_item {
                        assert_eq!(
                            client
                                .get_db_item(&real_db_name, "1")
                                .await?,
                            vec![record]
                        );
                    }
                    test get_db_index {
                        client.get_db_index(&real_db_name).await?;
                    }
                    test db_query {
                        use orbit_db_http_client::Query;
                        assert_eq!(
                            client.db_query(&real_db_name, Query {
                                propname: None,
                                comp: None,
                                values: vec![],
                            }).await?,
                            vec![record]
                        );
                    }
                }
            }
        }
    }
}
