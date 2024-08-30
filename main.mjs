import { 
    default as wasm, 
} from "./pkg/yo_yo.js";


if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|BB|PlayBook|IEMobile|Windows Phone|Kindle|Silk|Opera Mini/i.test(navigator.userAgent)) {

    alert("Вы используете мобильное устройство (телефон или планшет).")

} else alert("Вы используете ПК.")


wasm().then((module) => {});  