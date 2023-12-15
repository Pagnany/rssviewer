const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let rssFeedEl;

async function example_rss() {
  await invoke("example_feed")
    .then((message) => (rssFeedEl.textContent = message))
    .catch((error) => console.error(error));
}

window.addEventListener("DOMContentLoaded", () => {
  rssFeedEl = document.querySelector("#rss-feed");
  document.querySelector("#rss-refresh").addEventListener("submit", (e) => {
    e.preventDefault();
    example_rss();
  });
});
