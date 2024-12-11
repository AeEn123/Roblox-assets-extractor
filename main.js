var tab = ""
var soundsList = []
var imageList = []
const PARSER = new Worker('parser.js');
const CACHE_CHOOSER = document.getElementById('cacheDirectoryChooser')

function add_audio(filename, binaryData) {
    const main = document.getElementById('main');
    const details = document.createElement('details');

    const summary = document.createElement('summary');
    summary.textContent = filename;
    details.appendChild(summary);

    const blob = new Blob([binaryData], { type: 'audio/ogg' });
    const url = URL.createObjectURL(blob);

    // Create an audio element
    const audio = document.createElement('audio');
    audio.controls = true;
    audio.src = url; // Set the audio source
    details.appendChild(audio);

    main.appendChild(details);

    // Release the URL when the element is removed to free resources
    details.addEventListener('toggle', () => {
        if (!details.open) {
            URL.revokeObjectURL(url);
        }
    });
}

function add_image(filename, binaryData) {
    const main = document.getElementById('main');
    const details = document.createElement('details');

    const summary = document.createElement('summary');
    summary.textContent = filename;
    details.appendChild(summary);

    const blob = new Blob([binaryData], { type: 'image/png' });
    const url = URL.createObjectURL(blob);

    // Create image element
    const img = document.createElement('img');
    img.src = url;
    img.alt = filename;
    img.style.width = '40em';
    img.style.height = 'auto';
    details.appendChild(img);

    main.appendChild(details);

    // Release the URL when the element is removed to free resources
    details.addEventListener('toggle', () => {
        if (!details.open) {
            URL.revokeObjectURL(url);
        }
    });
}


function list_sounds() {
    let index = 0;
    const batchSize = 10; // Number of items to process per batch

    function processBatch() {
        if (tab != "sounds") {
            return
        }
        const start = index;
        const end = Math.min(index + batchSize, soundsList.length);

        for (let i = start; i < end; i++) {
            const asset = soundsList[i];
            add_audio(asset[0],asset[1]);
        }

        index = end;

        if (index < soundsList.length) {
            setTimeout(processBatch, 10); // Schedule the next batch after a short delay
        } else {
            console.log('All sounds listed!');
        }
    }

    processBatch(); // Start processing
}

function list_images() {
    let index = 0;
    const batchSize = 10; // Number of items to process per batch

    function processBatch() {
        if (tab != "images") {
            return
        }
        const start = index;
        const end = Math.min(index + batchSize, imageList.length);

        for (let i = start; i < end; i++) {
            const asset = imageList[i];
            add_image(asset[0],asset[1]);
        }

        index = end;

        if (index < imageList.length) {
            setTimeout(processBatch, 10); // Schedule the next batch after a short delay
        }
    }

    processBatch(); // Start processing
}


// Tab switching
function sounds() {
    tab = "sounds"
    document.getElementById('soundstab').className = "active" // Select
    document.getElementById('imagestab').className = "inactive"
    document.getElementById('main').innerHTML = ""

    list_sounds()
}

function image() {
    tab = "images"
    document.getElementById('soundstab').className = "inactive"
    document.getElementById('imagestab').className = "active" // Select
    document.getElementById('main').innerHTML = ""

    list_images()
}

CACHE_CHOOSER.addEventListener('change', function(event) {
    CACHE_CHOOSER.parentElement.innerHTML = ""
    PARSER.postMessage(Array.from(event.target.files))
    PARSER.onmessage = function(event) {
        const asset = event.data
        if (asset[0] == "sounds") {
            soundsList.push([asset[1], asset[2]])
        } else if (asset[0] == "images") {
            imageList.push([asset[1], asset[2]])
        }

    }
    document.getElementById('main').innerHTML = "Begin by clicking a tab above"

})