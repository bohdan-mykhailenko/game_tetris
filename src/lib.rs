use js_sys::{ Function, Reflect };
use wasm_bindgen::{ prelude::Closure, JsValue, JsCast, UnwrapThrowExt };
use wasm_react::{
    c,
    export_components,
    h,
    hooks::{ use_effect, use_state, Deps },
    props::Style,
    Component,
};
use web_sys::window;
use tetris::Tetris;

mod tetris;
mod shape;

pub struct App {
    width: u32,
    height: u32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(App {
            width: Reflect::get(&value, &"width".into())?.as_f64().unwrap_or(10.0) as u32,
            height: Reflect::get(&value, &"height".into())?.as_f64().unwrap_or(30.0) as u32,
        })
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode {
        let tetris = use_state(|| Tetris::new(self.height, self.width));

        use_effect(
            {
                let tetris = tetris.clone();

                move || {
                    let tick_closure = Closure::new({
                        let mut tetris = tetris.clone();

                        move || {
                            tetris.set(|mut tetris| {
                                tetris.tick();
                                tetris
                            });
                        }
                    });

                    let handle = window()
                        .unwrap_throw()
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                            500
                        )
                        .unwrap_throw();

                    move || {
                        drop(tick_closure);
                        window().unwrap_throw().clear_interval_with_handle(handle);
                    }
                }
            },
            Deps::none()
        );

        h!(div)
            .style(
                &Style::new()
                    .display("inline-grid")
                    .grid_template(
                        format!("repeat({}, 1em) / repeat({}, 1em)", self.height, self.width)
                    )
                    .border("1px solid grey")
                    .outline("none")
            )

            .build(
                c![
                    ..tetris
                        .value()
                        .iter_positions()
                        .map(|pos| {
                            let typ = tetris.value().get(pos);

                            h!(div).build(c![typ.unwrap_or_default()])
                        })
                ]
            )
    }
}

export_components!(App);
