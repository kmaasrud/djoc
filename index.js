import init, { Djoc } from "./djoc_sandbox.js";
await init();

function update_url(params) {
  const path = new URL(window.location.href).pathname;
  let newState = path;

  for (const [key, value] of Object.entries(params)) {
    if (value != "" && value != null) {
      newState += (newState == path ? "?" : "&") + key + "=" +
        encodeURIComponent(value).replace(/%0A$/, "");
    }
  }

  window.history.replaceState("object or string", "Title", newState);
}

function render(params) {
  djoc.update_doc(params.content);
  djoc.set_title(params.title);
  djoc.set_author(params.author);

  if (format.value == "preview") {
    output.className = "";
    output.innerHTML = djoc.render();
  } else if (format.value == "latex") {
    output.className = "latex";
    output.innerText = djoc.render_latex();
  } else if (format.value == "html") {
    output.className = "html";
    output.innerText = djoc.render();
  }

  hljs.highlightAll();
}

function update() {
  params.title = title.value || "";
  params.author = author.value || "";
  params.content = input.innerText;

  render(params);
  update_url(params);
}

const title = document.getElementById("djoc-title");
const author = document.getElementById("djoc-author");
const input = document.getElementById("djoc-input");

const format = document.getElementById("djoc-format");
const output = document.getElementById("djoc-output");

const params = Object.fromEntries(new URLSearchParams(window.location.search));

title.value = params.title || "";
author.value = params.author || "";
input.innerText = params.content || "";

const djoc = Djoc.new();

update();

input.onkeyup = update;
format.onchange = update;
title.onkeyup = update;
author.onkeyup = update;

// auto focus on input on load
setTimeout(() => {
  input.focus();
}, 0);
