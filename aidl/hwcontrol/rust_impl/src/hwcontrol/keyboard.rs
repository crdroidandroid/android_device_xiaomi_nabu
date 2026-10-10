//
// Copyright (C) 2024 Harshit Jain <dev-harsh1998@hotmail.com>
//
// SPDX-License-Identifier: Apache-2.0
//
use super::sysfs::SysFSManager;
use super::err_mgr::mng_err;

/// This function sets the keyboard state.
/// input: enable: bool
/// output: Result<(), bool>
/// The function returns Ok(()) if the operation was successful, otherwise it returns Err(false).
pub fn set_keyboard(enable: bool) -> Result<(), bool> {
    let sysfs_keyboard_node = "/sys/devices/platform/soc/soc:xiaomi_keyboard/xiaomi_keyboard_conn_status";
    match SysFSManager::write_to_sysfs(sysfs_keyboard_node, if enable { "enable_keyboard" } else { "disable_keyboard" }) {
        Ok(_) => Ok(()),
        Err(e) => {
            mng_err(e, sysfs_keyboard_node);
            Err(false)
        }
    }
}

// This function gets the keyboard state.
// output: Result<bool, bool>
// The function returns Ok(true) if the keyboard is connected, otherwise it returns Ok(false).
// Note: the node reports connection status as "1" (connected) / "0" (disconnected),
// not the enable/disable strings that are written to it to control the keyboard.
pub fn get_keyboard() -> Result<bool, bool> {
    let sysfs_keyboard_node = "/sys/devices/platform/soc/soc:xiaomi_keyboard/xiaomi_keyboard_conn_status";
    match SysFSManager::read_from_sysfs(sysfs_keyboard_node) {
        Ok(data) => {
            if data.trim() == "1" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => {
            mng_err(e, sysfs_keyboard_node);
            Err(false)
        }
    }
}