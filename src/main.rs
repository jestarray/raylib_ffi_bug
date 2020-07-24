#![allow(non_snake_case)]
use raylib::ffi;
use raylib::prelude::*;
use std::ffi::CString;
//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn main() {
    // Initialization
    //---------------------------------------------------------------------------------------
    let screenWidth = 690;
    let screenHeight = 560;


    unsafe {
        ffi::InitWindow(
            screenWidth,
            screenHeight,
            CString::new("raygui test").unwrap().as_ptr() as *const _,
        );
        ffi::SetTargetFPS(60);
    }

    let mut spinner001Value = 0;
    let mut spinnerEditMode = false;

    let mut exitWindow = false;
    let mut buff = CString::new("test").unwrap().as_bytes();
    while !exitWindow {
        use raylib::consts::guiIconName::*;
        use raylib::consts::GuiControl::*;
        use raylib::consts::GuiControlProperty::*;
        use raylib::consts::GuiControlState::*;
        use raylib::consts::GuiDefaultProperty::*;
        use raylib::consts::GuiTextAlignment::*;
        use raylib::consts::KeyboardKey::*;
        exitWindow = unsafe { ffi::WindowShouldClose() };

        unsafe {
            ffi::BeginDrawing();
            ffi::ClearBackground(Color::WHITE.into());
        }

        unsafe {
            ffi::GuiSetStyle(
                TEXTBOX as i32,
                TEXT_ALIGNMENT as i32,
                GUI_TEXT_ALIGN_CENTER as i32,
            );
        
        }
        unsafe {
            ffi::DrawCircle(
                50,
                50,
                5.0,
                if spinnerEditMode {
                    Color::BLUE.into()
                } else {
                    Color::RED.into()
                },
            );
            //FFI CALL DOES NOT CALL GUIVALUEBOX CORRECTLY
            //BOOL TURNS INTO INT ON THE C side
            let spinny = ffi::GuiValueBox(
                ffi::Rectangle {
                    x: 25.0 as f32,
                    y: 135.0 as f32,
                    width: 125.0 as f32,
                    height: 30.0 as f32,
                },
                0 as *const ::std::os::raw::c_char,
                &mut spinner001Value,
                0,
                100,
                spinnerEditMode,
            );
            if spinny {
                spinnerEditMode = if spinnerEditMode {
                    false
                } else {true};
                dbg!(spinnerEditMode);
            }
        }
        unsafe {
            ffi::EndDrawing();
        }
    }
}
