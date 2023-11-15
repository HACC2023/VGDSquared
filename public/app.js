const menu = document.getElementById('menuPic')
const xPic = document.getElementById('xPic')
const tab = document.getElementById('tab')

menu.addEventListener("click", function(){
    menu.style.display = "none"
    tab.style.display = "block"
})

xPic.addEventListener("click", function(){
    tab.style.display = "none"
    menu.style.display = "block"
})

// This is for unpacking the timestamp from the server
function unpack_timestamp(timestamp) {
    const [year, day, hour, minute, second, nanosecond] = timestamp;
    return Date.UTC(year, 0, day, hour, minute, second, nanosecond / 1_000_000);
}
