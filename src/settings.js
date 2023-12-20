const { invoke } = window.__TAURI__.tauri;

let rssFeedChannelsEl;

async function delete_rss_feed(id) {
  await invoke("delete_rss_feed_channel_from_database", { id })
    .then(() => {
      show_rss_feed();
    })
    .catch((error) => console.error(error));
}

async function add_rss_feed() {
  let name = document.querySelector("#rss-name").value;
  let url = document.querySelector("#rss-url").value;
  let active = true;
  await invoke("insert_rssfeed_into_databese", { name, url, active })
    .then(() => {
      show_rss_feed();
    })
    .catch((error) => console.error(error));
}

async function show_rss_feed() {
  let rss_feed_channels = await invoke("get_rss_feed_channel_from_database");
  rssFeedChannelsEl.innerHTML = "";
  for (let i = 0; i < rss_feed_channels.length; i++) {
    let article = document.createElement("article");
    article.innerHTML = `
                <h4>${rss_feed_channels[i].name}</h4>
                <p>${rss_feed_channels[i].url}</p>
                <input type="checkbox" ${
                  rss_feed_channels[i].active ? "checked" : ""
                } /> Active
                <br />
                <button class="delete-button">Delete</button>
        `;
    article
      .querySelector(".delete-button")
      .addEventListener("click", () =>
        delete_rss_feed(rss_feed_channels[i].id)
      );
    rssFeedChannelsEl.appendChild(article);
    rssFeedChannelsEl.innerHTML += "<br />";
  }
}

window.addEventListener("DOMContentLoaded", () => {
  rssFeedChannelsEl = document.querySelector("#rss-feed-channels");
  show_rss_feed();

  let form = document.querySelector("#rss-settings");
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    add_rss_feed();
  });
});
