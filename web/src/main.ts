// Main Entrypoint
function main(): HTMLElement {
  const HTML_MAIN: HTMLElement = document.createElement("main");
  const HTML_PARAGRAPH: HTMLParagraphElement = document.createElement("p");
  
  HTML_MAIN
  HTML_PARAGRAPH
  HTML_PARAGRAPH.innerText = String("Hyaena Technologies");
  HTML_MAIN.appendChild(HTML_PARAGRAPH);

  return document.body.appendChild(HTML_MAIN); 
}

main();

