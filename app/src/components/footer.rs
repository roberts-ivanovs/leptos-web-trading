use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="mt-32 bg-gray-900 sm:mt-56" aria-labelledby="footer-heading">
            <h2 id="footer-heading" class="sr-only">
                "Footer"
            </h2>
            <div class="mx-auto max-w-7xl px-6 pb-8 pt-8 sm:pt-8 lg:px-8">
                <div class="flex justify-center pt-8">
                    <div class="flex flex-col items-center">
                        <div class="flex">
                            <a
                                href="https://github.com/roberts-ivanovs/leptos-web-trading"
                                class="text-gray-500 hover:text-gray-400"
                            >
                                <span class="sr-only">"GitHub"</span>
                                <svg
                                    class="h-20"
                                    fill="currentColor"
                                    viewBox="0 0 24 24"
                                    aria-hidden="true"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                                        clip-rule="evenodd"
                                    ></path>
                                </svg>
                            </a>
                        </div>
                        <ul class="flex space-x-4 mt-6">
                            <li>
                                <a
                                    href="/"
                                    class="text-sm leading-6 text-gray-300 hover:text-white"
                                >
                                    "Home"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="/market"
                                    class="text-sm leading-6 text-gray-300 hover:text-white"
                                >
                                    "Markets"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="/community"
                                    class="text-sm leading-6 text-gray-300 hover:text-white"
                                >
                                    "Community"
                                </a>
                            </li>
                        </ul>
                        <p class="text-xs leading-5 text-gray-400 md:order-1 mt-0">
                            "© 2023 Roberts Ivanovs. All rights reserved."
                        </p>
                    </div>
                </div>
            </div>
        </footer>
    }
}
