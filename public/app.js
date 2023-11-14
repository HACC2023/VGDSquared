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