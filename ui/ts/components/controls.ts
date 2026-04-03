export class Controls {
  private container: HTMLElement;
  public currentIndex = 0;
  public totalSteps = 0;
  public isPlaying = false;
  public speed = 1.0;

  private playBtn: HTMLButtonElement;
  private prevBtn: HTMLButtonElement;
  private nextBtn: HTMLButtonElement;
  private speedLabel: HTMLElement;

  constructor(
    containerId: string,
    private onPrev: () => void,
    private onNext: () => void,
    private onTogglePlay: () => void,
    private onSpeedChange: (speed: number) => void
  ) {
    this.container = document.getElementById(containerId)!;

    this.container.innerHTML = `
      <div class="flex-row">
        <button class="control-btn prev-btn">◀ Prev</button>
        <button class="control-btn play-btn">▶ Play</button>
        <button class="control-btn next-btn">▶▶ Next</button>
        <div class="spacer"></div>
        <label>Speed:</label>
        <input type="range" class="speed-slider" min="0.1" max="5.0" step="0.1" value="1.0">
        <span class="speed-label font-mono">1.0 steps/s</span>
      </div>
    `;

    this.prevBtn = this.container.querySelector('.prev-btn') as HTMLButtonElement;
    this.playBtn = this.container.querySelector('.play-btn') as HTMLButtonElement;
    this.nextBtn = this.container.querySelector('.next-btn') as HTMLButtonElement;
    const speedSlider = this.container.querySelector('.speed-slider') as HTMLInputElement;
    this.speedLabel = this.container.querySelector('.speed-label') as HTMLElement;

    this.prevBtn.addEventListener('click', () => {
      if (this.currentIndex > 0) this.onPrev();
    });

    this.nextBtn.addEventListener('click', () => {
      if (this.currentIndex < this.totalSteps - 1) this.onNext();
    });

    this.playBtn.addEventListener('click', () => {
      this.onTogglePlay();
      this.updateState(this.currentIndex, this.totalSteps, this.isPlaying);
    });

    speedSlider.addEventListener('input', (e) => {
      const target = e.target as HTMLInputElement;
      this.speed = parseFloat(target.value);
      this.speedLabel.textContent = `${this.speed.toFixed(1)} steps/s`;
      this.onSpeedChange(this.speed);
    });
  }

  public updateState(current: number, total: number, playing: boolean) {
    this.currentIndex = current;
    this.totalSteps = total;
    this.isPlaying = playing;

    this.prevBtn.disabled = this.currentIndex === 0;
    this.nextBtn.disabled = this.currentIndex >= this.totalSteps - 1;

    if (this.isPlaying) {
      this.playBtn.textContent = '⏸ Pause';
    } else {
      this.playBtn.textContent = '▶ Play';
    }
  }
}
