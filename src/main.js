const { writeTextFile, writeFile, BaseDirectory, createDir } =
  window.__TAURI__.fs;

const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let rssFeedEl;

let xml;

async function example_rss() {
  await invoke("example_feed")
    .then((message) => {
      let rssDate = "";
      message.forEach((item) => {
        rssDate += "<article>";
        rssDate += "<p>" + item.feed_name + "</p>";
        rssDate += "<h4>" + item.header + "</h4>";
        rssDate += "<p>" + item.description + "</p>";
        rssDate += "<img src='" + item.image + "'/>" + "<br />";

        rssDate +=
          '<a href="' +
          item.url +
          ' " target="_blank">' +
          "Link" +
          "</a>" +
          "<br />";

        rssDate += item.date + "<br />";
        rssDate += "</article>";
        rssDate += "<br/>";
      });

      rssFeedEl.innerHTML = rssDate;
    })
    .catch((error) => console.error(error));
}

window.addEventListener("DOMContentLoaded", () => {
  rssFeedEl = document.querySelector("#rss-feed");
  document.querySelector("#rss-refresh").addEventListener("submit", (e) => {
    e.preventDefault();
    example_rss();
  });

  setInterval(example_rss, 60000);
});
