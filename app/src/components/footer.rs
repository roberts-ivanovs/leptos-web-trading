use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="mt-32 bg-gray-900 sm:mt-56" aria-labelledby="footer-heading">
            <h2 id="footer-heading" class="sr-only">
                "Footer"
            </h2>
            <div class="mx-auto max-w-7xl px-6 pb-8 pt-16 sm:pt-8 lg:px-8 lg:pt-16">
                <div class="pb-16 border-b border-white/10 pt-8 sm:pb-16 lg:pb-16 lg:flex lg:items-center lg:justify-between">
                    <div>
                        <h3 class="text-sm font-semibold leading-6 text-white">
                            "Subscribe to our newsletter"
                        </h3>
                        <p class="mt-2 text-sm leading-6 text-gray-300">
                            "The latest news, articles, and resources, sent to your inbox weekly."
                        </p>
                    </div>
                    <form class="mt-6 sm:flex sm:max-w-md lg:mt-0">
                        <label for="email-address" class="sr-only">
                            "Email address"
                        </label>
                        <input
                            type="email"
                            name="email-address"
                            id="email-address"
                            autocomplete="email"
                            required
                            class="w-full min-w-0 appearance-none rounded-md border-0 bg-white/5 px-3 py-1.5 text-base text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:w-56 sm:text-sm sm:leading-6"
                            placeholder="Enter your email"
                        />
                        <div class="mt-4 sm:ml-4 sm:mt-0 sm:flex-shrink-0">
                            <button
                                type="submit"
                                class="flex w-full items-center justify-center rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                            >
                                "Subscribe"
                            </button>
                        </div>
                    </form>
                </div>
                <div class="flex justify-center pt-8">
                    <div class="flex flex-col items-center">
                        <div class="flex">
                            <a href="https://github.com/roberts-ivanovs/leptos-web-trading" class="text-gray-500 hover:text-gray-400">
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
                        <p class="text-xs leading-5 text-gray-400 md:order-1 mt-0">"&copy; 2023 Roberts Ivanovs. All rights reserved."</p>
                    </div>
                </div>
            </div>
        </footer>
    }
}
