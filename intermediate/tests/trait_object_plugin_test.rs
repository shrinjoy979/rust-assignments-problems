use intermediate::hard::trait_object_plugin::{AddOne, Double, PluginManager};

#[test]
fn test_plugin_chain() {
    let mut manager = PluginManager::new();
    manager.add(Box::new(AddOne)); // (x + 1)
    manager.add(Box::new(Double)); // (x + 1) * 2
    assert_eq!(manager.run_all(5), 12);
}

#[test]
fn test_empty_manager() {
    let manager = PluginManager::new();
    assert_eq!(manager.run_all(10), 10);
}

#[test]
fn test_plugin_names() {
    let add = AddOne;
    use intermediate::hard::trait_object_plugin::Plugin;
    assert_eq!(add.name(), "AddOne");
}
