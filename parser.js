const headers = {
    "sounds": ["OggS", "ID3"],
    "images": ["PNG", "WEBP"],
    // "ktx-files": ["KTX"], // Not used in JavaScript version
    // "rbxm-files": ["<roblox!"]
};

const offsets = {
    "PNG": 1,
    "KTX": 1,
    "WEBP": 8
};

const headers_keys = Object.keys(headers);

onmessage = function(event) {
    const files = event.data;

    files.forEach(file => {
        const reader = new FileReader();
        reader.onload = function(e) {
            const arrayBuffer = e.target.result;
            const uint8Array = new Uint8Array(arrayBuffer); // Convert to Uint8Array for binary operations

            // Decode first 2048 bytes to a string for header matching
            const headerText = new TextDecoder().decode(uint8Array.slice(0, 2048));

            let found = false;

            // Match headers
            headers_keys.forEach(type => {
                if (found) return;

                headers[type].forEach(header => {
                    if (headerText.includes(header)) {
                        const index = headerText.indexOf(header);
                        let offset = offsets[header] || 0;

                        // Slice the ArrayBuffer starting from the header position (adjusted by offset)
                        const slicedBuffer = arrayBuffer.slice(Math.max(0, index - offset));

                        // Send matched file info to the main thread
                        postMessage([type, file.name, slicedBuffer]);
                        found = true;
                    }
                });
            });
        };

        reader.onerror = function(e) {
            console.error(`Error reading file: ${file.name}`, e);
        };

        reader.readAsArrayBuffer(file); // Read file as binary data
    });
};
