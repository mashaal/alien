class VideoWaste extends HTMLElement {
  constructor() {
    super();
  }
  connectedCallback() {
    const canvas = this.shadowRoot.querySelector("canvas.one");
    const ctx = canvas.getContext("2d");
    const canvas2 = this.shadowRoot.querySelector("canvas.two");
    const ctx2 = canvas2.getContext("2d");
    const video = this.shadowRoot.querySelector("video");

    video.addEventListener("play", () => {
      canvas.width = video.offsetWidth;
      canvas.height = video.offsetHeight;
      canvas2.width = video.offsetWidth;
      canvas2.height = video.offsetHeight;

      function step() {
        ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
        ctx2.drawImage(canvas, 0, 0, canvas.width, canvas.height);

        requestAnimationFrame(step);
      }
      requestAnimationFrame(step);
    });
  }
}
customElements.define("video-waste", VideoWaste);
