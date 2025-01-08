const form = document.getElementById("form");
const log = document.getElementById("log");
const regex = /^.*\.txt$/gm;
let xhr = new XMLHttpRequest();

form.onsubmit = (e) => {
    let fileTitleInput = document.getElementById("file-title");
    let fileTextArea = document.getElementById("file-text");

    e.preventDefault();
   
    if (!regex.test(fileTitleInput.value)) {
        console.log("WRONGGGGGGGGGGGGGGGGG");
        fileTitleInput.setCustomValidity("You need to enter a file name!");
    }


    xhr.open("PUT", `/${fileTitleInput.value}`);
    xhr.setRequestHeader('Content-Type', 'application/json; charset=utf-8');
    xhr.send(JSON.stringify({
        "file-title": fileTitleInput.value,
        "file-text": fileTextArea.value
    }));

    xhr.onload = function() {
        if (xhr.status == 201) 
            alert("File was succesefuly created");
    }

    log.textContent = `
        Form Submitted! Timestamp: ${e.timeStamp},
        File title: ${fileTitleInput.value},
        File content: ${fileTextArea.value}
    `;
}
