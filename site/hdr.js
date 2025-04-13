const logoImg = document.getElementById('logo-img');
logoImg.addEventListener('click', () => {
    window.location.href = 'index.html';
});

const screenshots = document.querySelectorAll('.screenshot');
screenshots.forEach(screenshot => {
    let isEnlarged = false; // Флаг для отслеживания состояния
    screenshot.addEventListener('click', () => {
        isEnlarged = !isEnlarged; // Меняем состояние флага
        screenshot.classList.toggle('enlarged');
    });
});

const menuToggle = document.getElementById('menu-toggle');
const mainMenu = document.getElementById('mainMenu');

menuToggle.addEventListener('click', () => {
    mainMenu.classList.toggle('show');
});