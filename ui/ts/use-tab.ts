import { invoke } from "@tauri-apps/api/core";
import { events } from "./main";
import { KeystreamGrid } from "./components/grid";

export interface EncryptResponse {
  ciphertext: string;
  keystream_bytes: number[];
}

export function initUseTab() {
  const inputEl = document.getElementById('use-input') as HTMLTextAreaElement;
  const hexCheckEl = document.getElementById('use-hex-check') as HTMLInputElement;
  const encryptBtn = document.getElementById('use-encrypt-btn') as HTMLButtonElement;
  const outputEl = document.getElementById('use-output') as HTMLTextAreaElement;
  const gridContainer = document.getElementById('use-keystream-grid') as HTMLElement;
  const helpLink = document.getElementById('use-help-link') as HTMLAnchorElement;

  if (!inputEl || !encryptBtn || !outputEl || !gridContainer) return;

  const grid = new KeystreamGrid(gridContainer);
  let isLoading = false;

  const encrypt = async () => {
    if (isLoading) return;
    const plaintext = inputEl.value;
    if (!plaintext) return;

    isLoading = true;
    encryptBtn.disabled = true;
    encryptBtn.textContent = "Encrypting...";

    try {
      const hex_output = hexCheckEl.checked;
      const response = await invoke<EncryptResponse>('encrypt_decrypt', {
        plaintext,
        hexOutput: hex_output
      });

      outputEl.value = response.ciphertext;
      grid.update(response.keystream_bytes);
    } catch (e) {
      console.error(e);
      outputEl.value = `Error: ${e}`;
    } finally {
      isLoading = false;
      encryptBtn.disabled = false;
      encryptBtn.textContent = "🔒 Encrypt / Decrypt";
    }
  };

  encryptBtn.addEventListener('click', encrypt);

  if (helpLink) {
    helpLink.addEventListener('click', (e) => {
      e.preventDefault();
      // Switch to learn tab
      document.querySelector('[data-tab="learn"]')?.dispatchEvent(new Event('click'));
      // Emit event for learn tab to pick up
      events.emit('navigate_learn_xor', inputEl.value);
    });
  }
}
