use leptos::{*, leptos_dom::console_log};
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (grid, set_grid) = create_signal(cx, [[0; 50]; 50]);
    let mut starter = [[0; 50]; 50];
    starter[22][23] = 1;
    starter[23][22] = 1;
    starter[23][23] = 1;
    starter[23][24] = 1;
    set_grid(starter);

    create_effect(cx, move |_| {
        // console_log("checking neighbors");
        let mut new_grid = grid.get();
        for i in 1..49 {
            for j in 1..49 {
                let mut living_neighbors = 0;
                // above
                living_neighbors += grid.get()[i-1][j-1];
                living_neighbors += grid.get()[i-1][j];
                living_neighbors += grid.get()[i-1][j+1];
                // same row
                living_neighbors += grid.get()[i][j-1];
                living_neighbors += grid.get()[i][j+1];
                // below
                living_neighbors += grid.get()[i+1][j-1];
                living_neighbors += grid.get()[i+1][j];
                living_neighbors += grid.get()[i+1][j+1];

                if i == 22 && j == 23 {
                    console_log(format!("{} {} {}", i, j, living_neighbors).as_str());
                }

                if grid.get()[i][j] == 0 && living_neighbors == 3 {
                    console_log("alived");
                    new_grid[i][j] = 1;
                } else if grid.get()[i][j] == 1 {
                    if living_neighbors == 2 || living_neighbors == 3 {
                        console_log("stays alive");
                        new_grid[i][j] = 1;
                    } else {
                        console_log("deaded");
                        new_grid[i][j] = 0;
                    }
                }
            }
        }
        set_grid(new_grid);
    });

    view! { cx,
        <Title text="Conway's Game of Life"/>
        <main class="w-screen h-screen bg-black p-5 flex flex-col justify-center items-center">
        <For each={move || grid.get()} key={move |_| 7} view=move |cx, row| {
           view! { cx,
               <div class="flex flex-row">
                    <For each={move || row} key={move |_| 7} view=move |cx, n| {
                       view! {cx,
                           {if n == 0 {
                               view! { cx, <div class="p-2 bg-black border border-gray-700"></div> }
                           } else {
                               view! { cx, <div class="p-2 bg-white border border-gray-700"></div> }
                           }}
                       }
                     }
                    />
               </div>
           }
         }
        />
        </main>
    }
}
