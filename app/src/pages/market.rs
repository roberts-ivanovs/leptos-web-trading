mod market_list;
mod ladder_view;

use ladder_view::LadderView;
use leptos::*;
use market_list::MarketList;

use leptos_meta::*;

use crate::layout::DefaultLayout;

#[component]
pub fn MarketPage(cx: Scope) -> impl IntoView {
    view! { cx,
            <Title text="Markets"/>
            <DefaultLayout>
            <div class="flex min-h-full flex-col">

      <div class="mx-auto flex w-full max-w-7xl items-start gap-x-8 px-4 py-10 sm:px-6 lg:px-8">
        <aside class="sticky top-8 hidden shrink-0 lg:block">
                <MarketList/>

        //   <!-- Left column area -->
        </aside>

        <main class="flex-1">
                <LadderView/>
        //   <!-- Main area -->
        </main>

        <aside class="sticky top-8 hidden w-96 shrink-0 xl:block">
                <LadderView/>
        //   <!-- Right column area -->
        </aside>
      </div>
    </div>

            </DefaultLayout>
        }
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    market_list::register_server_functions();
}
