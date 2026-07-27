const toggle = document.getElementById("dark-mode");
const theme_img = document.getElementById("theme-img");
let editable_img;


const ICON_LIGHT = "/assets/enable-light-mode.svg";
const ICON_DARK = "/assets/enable-dark-mode.svg";

// Changes the theme depending on the theme the user has
function setTheme(theme) {
	// For all edit/pencil icons on home screen
	if (
		window.location.pathname === '/' ||
		window.location.pathname === '/index.html'
	) {
		editable_img = Array.from(document.getElementsByClassName("edit-button"));
		if (editable_img != undefined) {
			editable_img.forEach((img) => {
				const dark_icon = "/assets/editable-board-dark.svg";
				const light_icon = "/assets/editable-board-light.svg";
				img.src = theme === "light" ? light_icon : dark_icon;
			});
		}
	}
	document.body.classList.toggle("light", theme === "light");
	theme_img.src = theme === "light" ? ICON_DARK : ICON_LIGHT;
	sessionStorage.setItem("theme", theme);
}
// Caches the user's theme preference
const saved = sessionStorage.getItem("theme");
if (saved === "light" || saved === "dark") {
	setTheme(saved);
} else {
	const prefersLight = window.matchMedia(
		"(prefers-color-scheme: light)",
	).matches;
	setTheme(prefersLight ? "light" : "dark");
}

toggle.addEventListener("click", (e) => {
	e.preventDefault(); // prevents jumping due to href
	const next = document.body.classList.contains("light") ? "dark" : "light";
	setTheme(next);
});