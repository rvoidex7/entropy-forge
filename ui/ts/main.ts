export function initTabs() {
  const tabBtns = document.querySelectorAll('.tab-btn');
  const tabPanels = document.querySelectorAll('.tab-panel');

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      // Remove active from all
      tabBtns.forEach(b => b.classList.remove('active'));
      tabPanels.forEach(p => p.classList.add('hidden'));

      // Add active to clicked
      btn.classList.add('active');
      const targetId = `tab-${btn.getAttribute('data-tab')}`;
      const targetPanel = document.getElementById(targetId);
      if (targetPanel) {
        targetPanel.classList.remove('hidden');
      }
    });
  });
}

// Simple pub-sub for cross-tab communication (like the Help link)
export const events = {
  listeners: {} as Record<string, Function[]>,
  on(event: string, callback: Function) {
    if (!this.listeners[event]) this.listeners[event] = [];
    this.listeners[event].push(callback);
  },
  emit(event: string, data?: any) {
    if (this.listeners[event]) {
      this.listeners[event].forEach(cb => cb(data));
    }
  }
};

import { initUseTab } from './use-tab';
import { initTestTab } from './test-tab';
import { initBenchTab } from './bench-tab';
import { initLearnTab } from './learn';

document.addEventListener('DOMContentLoaded', () => {
  initTabs();
  initUseTab();
  initTestTab();
  initBenchTab();
  initLearnTab();
});
