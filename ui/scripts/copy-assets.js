const fs = require('fs');
const path = require('path');

// Create dist/styles directory
const stylesDir = path.join(__dirname, '..', 'dist', 'styles');
if (!fs.existsSync(stylesDir)) {
  fs.mkdirSync(stylesDir, { recursive: true });
}

// Copy index.html
fs.copyFileSync(
  path.join(__dirname, '..', 'index.html'),
  path.join(__dirname, '..', 'dist', 'index.html')
);
console.log('✓ Copied index.html');

// Copy CSS files
const stylesSourceDir = path.join(__dirname, '..', 'styles');
if (fs.existsSync(stylesSourceDir)) {
  const files = fs.readdirSync(stylesSourceDir);
  files.forEach(file => {
    if (file.endsWith('.css')) {
      fs.copyFileSync(
        path.join(stylesSourceDir, file),
        path.join(stylesDir, file)
      );
      console.log(`✓ Copied styles/${file}`);
    }
  });
}

console.log('✓ Assets copied successfully');
