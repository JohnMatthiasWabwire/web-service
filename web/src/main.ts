// Main Entrypoint
function main(): HTMLHtmlElement {
  const HTML_DOCUMENT_BODY: HTMLBodyElement = document.createElement("body");
  const HTML_DOCUMENT_METADATA_HEADER: HTMLHeadElement = document.createElement("head");    
  const HTML_MAIN: HTMLElement = document.createElement("main");
  const HTML_METADATA: HTMLMetaElement = document.createElement("meta");
  const HTML_PARAGRAPH: HTMLParagraphElement = document.createElement("p");
    
  HTML_DOCUMENT_METADATA_HEADER  
  HTML_METADATA
  HTML_METADATA.name = String("main-index");
  HTML_METADATA.content = String("width=device-width");
  HTML_DOCUMENT_METADATA_HEADER.appendChild(HTML_METADATA);

  document.appendChild(HTML_DOCUMENT_METADATA_HEADER);
  
  HTML_DOCUMENT_BODY  
  HTML_MAIN
  HTML_PARAGRAPH
  HTML_PARAGRAPH.innerText = String("Hyaena Technologies");
  HTML_MAIN.appendChild(HTML_PARAGRAPH);
  HTML_DOCUMENT_BODY.appendChild(HTML_MAIN);

  document.appendChild(HTML_DOCUMENT_BODY); 
}

main();

