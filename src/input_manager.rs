use std::collections::HashMap;
pub fn key_down(input_manager: &mut HashMap<String, bool>, keycode: String) {
    
    if !input_manager.contains_key(&keycode) {
        input_manager.entry(keycode).or_insert(true);
    }
    else
    {
        if let Some(x) = input_manager.get_mut(&keycode) {
            *x = true;
        }
    }
}

pub fn key_up(input_manager:&mut HashMap<String, bool>, keycode: String) {
    
    if !input_manager.contains_key(&keycode) {
        input_manager.entry(keycode).or_insert(false);
    }else{
        if let Some(x) = input_manager.get_mut(&keycode) {
            *x = false;
        }
    }
}

pub fn is_key_pressed(input_manager: &HashMap<String, bool>, value: &str) -> bool {
    input_manager.contains_key(&value.to_string()) && input_manager.get(&value.to_string())==Some(&true)
}