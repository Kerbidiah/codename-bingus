const { invoke } = window.__TAURI__.core;

const toggle = document.getElementById("dark-mode");
const theme_img = document.getElementById("theme-img");
const editable_img = Array.from(document.getElementsByClassName("editable"));

const ICON_LIGHT = "/assets/enable-light-mode.svg";
const ICON_DARK = "/assets/enable-dark-mode.svg";

// Changes the theme depending on the theme the user has
function setTheme(theme) {
	document.body.classList.toggle("light", theme === "light");
	theme_img.src = theme === "light" ? ICON_DARK : ICON_LIGHT;
	if (editable_img != null) {
		editable_img.forEach((img) => {
			const dark_icon = "/assets/editable-board-dark.svg";
			const light_icon = "/assets/editable-board-light.svg";
			img.src = theme === "light" ? light_icon : dark_icon;
		});
	}
	localStorage.setItem("theme", theme);
}
// Caches the user's theme preference
const saved = localStorage.getItem("theme");
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

// Creates the card in the DOM
function draw_card(name, owner) {
	const card_wrapper = document.createElement("div");
	card.className = "card-wrapper";

	const name_wrapper = document.createElement("div");
	name_wrapper.className = "name-wrapper";
	const card_name = document.createElement("a");
	card_name.textContent = name;
	name_wrapper.appendChild(card_name);

	const button_wrapper = document.createElement("div");
	button_wrapper.className = "button-wrapper";
	if (owner === true) {
		// TODO: Handle routing the user to the proper board (all items screen)
		const edit_anchor = document.createElement("a");
		const edit_img = document.createElemnt("img");
		const EDIT_ICON_PATH = "/assets/editable-board-dark.svg";
		edit_img.src = EDIT_ICON_PATH;
		edit_anchor.appendChild(edit_img);

		button_wrapper.appendChild(edit_anchor);
	}
	// TODO: Handle routing the user to the proper board (play)
	const play_anchor = document.createElemnt("a");
	const play_img = document.createElement("img");
	const PLAY_ICON_PATH = "/assets/play-board.svg";
	play_img.src = PLAY_ICON_PATH;
	play_anchor = appendChild(play_img);

	button_wrapper.appendChild(play_anchor);
	card_wrapper.appendChild(name_wrapper);
	card_wrapper.appendChild(button_wrapper);
}
let boards;
try {
	boards = await invoke('get_bingos');
} catch (error) {
	// If the function doesn't exist or it fails the boards should be empty
	boards = [];
}
if (boards.length === 0) {
	const cards = document.getElementById("bingo-cards");

	const create_board_button = document.createElement("button");
	create_board_button.id = "create-board";
	create_board_button.textContent = "Get Started";
	create_board_button.addEventListener("click", () => {
		window.location.href = "editable-board/editable-board.html";
	});
	cards.appendChild(create_board_button);
} else {
	for (const elem in boards) {
		// TODO: Render the cards of the boards.
		draw_card(elem.city, elem.owner);
	}
}
