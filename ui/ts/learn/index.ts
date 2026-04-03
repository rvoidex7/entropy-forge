import { initXorVisualizer } from './xor';
import { initEntropyVisualizer } from './entropy';
import { initNistVisualizer } from './nist';

export function initLearnTab() {
  const subTabBtns = document.querySelectorAll('.sub-tab-btn');
  const subTabPanels = document.querySelectorAll('.sub-tab-panel');

  subTabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      subTabBtns.forEach(b => b.classList.remove('active'));
      subTabPanels.forEach(p => p.classList.add('hidden'));

      btn.classList.add('active');
      const targetId = `learn-${btn.getAttribute('data-subtab')}`;
      const targetPanel = document.getElementById(targetId);
      if (targetPanel) {
        targetPanel.classList.remove('hidden');
      }
    });
  });

  initXorVisualizer();
  initEntropyVisualizer();
  initNistVisualizer();
}
