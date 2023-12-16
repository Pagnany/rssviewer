const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let rssFeedEl;

let xml;

async function example_rss() {
  await invoke("example_feed")
    .then((message) => {
      let rssDate = "";
      let parser = new DOMParser();
      xml = parser.parseFromString(message, "text/xml");

      //rssFeedEl.textContent = xml.getElementsByTagName("item")[0].childNodes[0].nodeValue;
      let itemscount = xml.getElementsByTagName("item").length;
      for (let i = 0; i < itemscount; i++) {
        rssDate += "<article>";
        rssDate +=
          "<h4>" +
          xml.getElementsByTagName("item")[i].getElementsByTagName("title")[0]
            .childNodes[0].nodeValue +
          "</h4>";
        rssDate +=
          "<p>" +
          xml
            .getElementsByTagName("item")
            [i].getElementsByTagName("description")[0].childNodes[0].nodeValue +
          "</p>";
        let urllink = xml
          .getElementsByTagName("item")
          [i].getElementsByTagName("link")[0].childNodes[0].nodeValue;

        rssDate +=
          '<a href="' + urllink + ' " target="_blank">Link</a>' + "<br />";

        rssDate +=
          xml.getElementsByTagName("item")[i].getElementsByTagName("pubDate")[0]
            .childNodes[0].nodeValue + "<br />";
        /*
        rssDate +=
          xml.getElementsByTagName("item")[i].getElementsByTagName("guid")[0]
            .childNodes[0].nodeValue + "<br />";
        */
        rssDate += "</article>";
        rssDate += "<br />";
      }

      //rssFeedEl.textContent = rssDate;
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
});
