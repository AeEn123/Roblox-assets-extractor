async function setBackground() {    
    const fileResponse = await fetch("hashes.txt");
    const fileContent = await fileResponse.text();
    
    document.getElementById("background").textContent = fileContent
}

function update(event) {
    const background = document.getElementById("background");
    background.style.left = `${-window.scrollX}px`;
    background.style.top = `${-window.scrollY}px`;
}

function scrollToHash() {
    var hash = window.location.hash;
    if (hash) {
        var target = document.querySelector(hash);
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
            });
        }
    }
}

setBackground()
document.addEventListener('scroll', event => update(event));
window.addEventListener('load', scrollToHash);
