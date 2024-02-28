const { writeTextFile, writeFile, BaseDirectory, createDir } =
  window.__TAURI__.fs;

const { invoke } = window.__TAURI__.tauri;

let rssFeedEl;
let button_go_top;

async function load_rssfeeds() {
  await invoke("load_rssfeeds")
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
  invoke("create_database")
    .then((_) => {})
    .catch((error) => console.error(error));

  rssFeedEl = document.querySelector("#rss-feed");
  document.querySelector("#rss-refresh").addEventListener("submit", (e) => {
    e.preventDefault();
    load_rssfeeds();
  });

  button_go_top = document.getElementById("go-top");
  button_go_top.addEventListener("click", () => {
    window.scrollTo(0, 0);
  });

  load_rssfeeds();
  setInterval(load_rssfeeds, 60000);
});
