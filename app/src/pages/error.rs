use http::StatusCode;
use leptos::*;
use leptos_meta::*;

use crate::error_template::AppError;
use crate::layout::ErrorLayout;

#[component]
pub fn ErrorPage(cx: Scope, errors: Vec<AppError>) -> impl IntoView {
    view! { cx,
        <ErrorLayout>
            <Title text="Error!"/>
            <main class="relative isolate min-h-full">
                <img
                    src="https://images.unsplash.com/photo-1545972154-9bb223aac798?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&ixlib=rb-1.2.1&auto=format&fit=crop&w=3050&q=80&exp=8&con=-15&sat=-75"
                    alt=""
                    class="absolute inset-0 -z-10 h-full w-full object-cover object-top"
                />
                <div class="mx-auto max-w-7xl px-6 py-32 text-center sm:py-40 lg:px-8">
                    <p class="text-base font-semibold leading-8 text-white">404</p>
                    <For
                        each=move || { errors.clone().into_iter().enumerate() }
                        key=|(index, _error)| *index
                        view=move |cx, error| {
                            let error_code = error.1.status_code();
                            match error_code {
                                StatusCode::NOT_FOUND => {
                                    view! { cx,
                                        <div>
                                            <h1 class="mt-4 text-3xl font-bold tracking-tight text-white sm:text-5xl">
                                                "Page not found"
                                            </h1>
                                            <p class="mt-4 text-base text-white/70 sm:mt-6">
                                                "Sorry, we couldn’t find the page you’re looking for."
                                            </p>
                                        </div>
                                    }
                                }
                                _ => {
                                    let error_string = error.1.to_string();
                                    view! { cx,
                                        <div>
                                            <h1 class="mt-4 text-3xl font-bold tracking-tight text-white sm:text-5xl">
                                                {error_code.to_string()}
                                            </h1>
                                            <p class="mt-4 text-base text-white/70 sm:mt-6">"Error: " {error_string}</p>
                                        </div>
                                    }
                                }
                            }
                        }
                    />
                    <div class="mt-10 flex justify-center">
                        <a
                            href="/"
                            target="_blank"
                            class="text-sm font-semibold leading-7 text-white"
                        >
                            "Back to home"
                        </a>
                    </div>
                </div>
            </main>
        </ErrorLayout>
    }
}
