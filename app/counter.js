class Counter extends HTMLElement {
  connectedCallback() {
    const increment = () => {
      this.innerHTML = +this.innerHTML + 1;
    };
    const button = this.shadowRoot?.querySelectorAll("button")[0];
    if (!button) return;
    button.disabled = false;
    button.addEventListener("click", increment);
  }
}
customElements.define("duster-counter", Counter);
