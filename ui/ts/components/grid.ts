export class KeystreamGrid {
  private container: HTMLElement;
  private cells: HTMLElement[] = [];

  constructor(container: HTMLElement) {
    this.container = container;
    this.container.classList.add('keystream-grid');
    this.render();
  }

  private render() {
    this.container.innerHTML = '';
    this.cells = [];
    for (let i = 0; i < 64; i++) {
      const cell = document.createElement('div');
      cell.className = 'keystream-cell';
      cell.title = `Byte ${i}: Empty`;
      this.container.appendChild(cell);
      this.cells.push(cell);
    }
  }

  public update(bytes: number[]) {
    for (let i = 0; i < 64; i++) {
      const cell = this.cells[i];
      if (i < bytes.length) {
        const val = bytes[i];
        cell.style.backgroundColor = `rgb(${val}, ${val}, ${val})`;

        // Convert to hex (e.g. 0A)
        let hex = val.toString(16).toUpperCase();
        if (hex.length < 2) hex = '0' + hex;

        cell.title = `Byte ${i}: 0x${hex} (${val})`;
      } else {
        cell.style.backgroundColor = 'transparent';
        cell.title = `Byte ${i}: Empty`;
      }
    }
  }
}
