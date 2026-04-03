export class ProgressBar {
  private container: HTMLElement;
  private labelEl: HTMLElement;
  private fillEl: HTMLElement;
  private valueEl: HTMLElement;

  constructor(
    container: HTMLElement,
    public label: string,
    public tooltipText: string | null = null,
    public colorThresholds = { green: 0.8, orange: 0.6 }
  ) {
    this.container = container;

    this.container.classList.add('progress-bar-container', 'flex-row');

    this.labelEl = document.createElement('div');
    this.labelEl.className = 'progress-label';
    this.labelEl.textContent = label;

    const trackEl = document.createElement('div');
    trackEl.className = 'progress-track';

    this.fillEl = document.createElement('div');
    this.fillEl.className = 'progress-fill';
    trackEl.appendChild(this.fillEl);

    this.valueEl = document.createElement('div');
    this.valueEl.className = 'progress-value';

    this.container.appendChild(this.labelEl);
    this.container.appendChild(trackEl);
    this.container.appendChild(this.valueEl);

    if (tooltipText) {
      const tooltip = document.createElement('span');
      tooltip.className = 'tooltip-icon';
      tooltip.textContent = 'ℹ';
      tooltip.setAttribute('data-tooltip', tooltipText);
      this.container.appendChild(tooltip);
    }
  }

  public update(value: number, max: number, displayValue: string) {
    const fraction = value / max;
    this.fillEl.style.width = `${Math.min(100, fraction * 100)}%`;
    this.valueEl.textContent = displayValue;

    if (fraction >= this.colorThresholds.green) {
      this.fillEl.style.backgroundColor = 'var(--accent-green)';
    } else if (fraction >= this.colorThresholds.orange) {
      this.fillEl.style.backgroundColor = 'var(--accent-orange)';
    } else {
      this.fillEl.style.backgroundColor = 'var(--accent-red)';
    }
  }
}
