
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