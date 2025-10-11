export {
  naviagtionMenu
};

// Navigation Menu
function naviagtionMenu(): HTMLElement {
  const HTML_HEADING_ONE: HTMLHeadElement = document.createElement("h1");
  const HTML_HEADING_TWO: HTMLHeadingElement = document.createElement("h2");
  const HTML_MAIN: HTMLElement = document.createElement("main");

  HTML_MAIN
  HTML_HEADING_ONE
  HTML_HEADING_ONE.innerText = String("Hyaena Technologies");
  HTML_MAIN.appendChild(HTML_HEADING_ONE);

  return document.body.appendChild(HTML_MAIN); 
}

naviagtionMenu();

