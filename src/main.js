const { invoke } = window.__TAURI__.core;

const toggle = document.getElementById("dark-mode");
const theme_img = document.getElementById("theme-img");
let editable_img;
const cards = document.getElementById("bingo-cards");

const ICON_LIGHT = "/assets/enable-light-mode.svg";
const ICON_DARK = "/assets/enable-dark-mode.svg";

// Changes the theme depending on the theme the user has
function setTheme(theme) {
	editable_img = Array.from(document.getElementsByClassName("edit-button"));
	document.body.classList.toggle("light", theme === "light");
	theme_img.src = theme === "light" ? ICON_DARK : ICON_LIGHT;
	if (editable_img != undefined) {
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
function draw_card(title, city, path, owner) {
	const card_wrapper = document.createElement("div");
	card_wrapper.className = "card-wrapper";

	const name_wrapper = document.createElement("div");
	name_wrapper.className = "name-wrapper";

	const card_anchor = document.createElement("div");

	const card_title = document.createElement("span");
	card_title.className = "card-title";

	const card_city = document.createElement("span");
	card_city.className = "card-city";

	card_title.textContent = title;
	card_city.textContent = city;

	card_anchor.appendChild(card_title);
	card_anchor.appendChild(card_city);
	name_wrapper.appendChild(card_anchor);

	const button_wrapper = document.createElement("div");
	button_wrapper.className = "button-wrapper";
	button_wrapper.id = path;
	if (owner === true) {
		const edit_anchor = document.createElement("a");
		const edit_img = document.createElement("img");
		const EDIT_ICON_PATH =
			saved === "light"
				? "/assets/editable-board-light.svg"
				: "/assets/editable-board-dark.svg";
		edit_img.src = EDIT_ICON_PATH;
		edit_img.className = "edit-button";
		edit_anchor.appendChild(edit_img);

		button_wrapper.appendChild(edit_anchor);
	}
	const play_anchor = document.createElement("a");
	const play_img = document.createElement("img");
	const PLAY_ICON_PATH = "/assets/play-board.svg";
	play_img.src = PLAY_ICON_PATH;
	play_img.className = "play-button";
	play_anchor.appendChild(play_img);

	button_wrapper.appendChild(play_anchor);
	card_wrapper.appendChild(name_wrapper);
	card_wrapper.appendChild(button_wrapper);
	cards.appendChild(card_wrapper);
}
if (
	window.location.pathname === "/" ||
	window.location.pathname === "/index.html"
) {
	let editable_boards_metadata; // All metadata for boards that are editable/playable
	let playable_boards_metadata; // All metadat for boards that are only playable
	try {
		editable_boards_metadata = await invoke("get_bingo_projects");
		playable_boards_metadata = await invoke("get_bingo_games");
	} catch (error) {
		editable_boards_metadata = [];
		playable_boards_metadata = [];
	}
	if (editable_boards_metadata.length === 0 && playable_boards_metadata.length === 0) {
		const create_board_button = document.createElement("button");
		create_board_button.id = "create-board";
		create_board_button.textContent = "Get Started";
		create_board_button.addEventListener("click", () => {
			window.location.href = "editable-board/editable-board.html";
		});
		cards.appendChild(create_board_button);
	} else {
		if (editable_boards_metadata != undefined) {
			for (const elem of editable_boards_metadata) {
				const items = elem[0];
				const path = elem[1];
				draw_card(items.title, items.city, path, true);
			}
		}
		if (playable_boards_metadata != undefined) {
			for (const elem of playable_boards_metadata) {
				const items = elem[0];
				const path = elem[1];
				draw_card(items.title, items.city, path, false);
			}
		}
	}
	document.addEventListener("click", (e) => {
		const button = e.target; // Gets either the edit/play buttons
		const button_wrapper = e.target.closest(".button-wrapper"); 
		if (!button_wrapper) return;
		else {
			localStorage.setItem("path", button_wrapper.id);
			if (button.className === "edit-button") {
				window.location.href = "./editable-board/editable-board.html";
			} else {
				window.location.href = "./generate-board/generate-board.html";
			}
		}
	});
}
