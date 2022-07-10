
import { LitElement, html } from "./lit.js";

class EpistemologyElement extends LitElement {
  messages = [];

  pending = false;

  constructor() {
    super();
  }

  async sendMessage() {
    const input = this.querySelector("#user-input");
    const message = input.value;
    const context = this.querySelector("#context").value;
    let newMessages = [...this.messages];
    newMessages.push({
      role: "user",
      content: message,
    });
    //filter outSystem context
    newMessages = newMessages.filter((message) => message.role !== "system");

    this.messages = newMessages;
    this.requestUpdate();

    // add new system context to front
    newMessages.unshift({
      role: "system",
      content: context,
    });

    input.value = "";
    const urlHost = window.location.host;
    const urlPath = "/api/chat";
    const url = `https://${urlHost}${urlPath}`;
    this.pending = true;
    this.requestUpdate();
    const response = await this.callChat(url, newMessages);
    newMessages.push(response);
    this.messages = newMessages.filter((message) => message.role !== "system");
    this.pending = false;
    window.scrollTo(0, document.body.scrollHeight);
    this.requestUpdate();
  }

  clearMessages() {