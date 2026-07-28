const { invoke } = window.__TAURI__.core;

const cards = document.getElementById("bingo-cards");

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
		const editButton = document.createElement("button");
		editButton.className = "edit";
		const editImage = document.createElement("img");
		const EDIT_ICON_PATH =
			sessionStorage.getItem("theme") === "light"
				? "/assets/editable-board-light.svg"
				: "/assets/editable-board-dark.svg";
		editImage.src = EDIT_ICON_PATH;
		editImage.className = "edit-button";
		editButton.appendChild(editImage);

		const exportButton = document.createElement("button");
		exportButton.className = "export";
		const exportImage = document.createElement("img");

		exportImage.src = "/assets/export.svg";
		exportImage.className = "export-button";
		exportButton.appendChild(exportImage);

		button_wrapper.appendChild(editButton);
		button_wrapper.appendChild(exportButton);
	} else {
		const playButton = document.createElement("button");
		playButton.className = "play";
		const playImage = document.createElement("img");
		playImage.src = "/assets/play-board.svg";
		playImage.className = "play-button";
		playButton.appendChild(playImage);

		button_wrapper.appendChild(playButton);
	}
	const deleteDiv = document.createElement("div");
	deleteDiv.className = "delete-wrapper";
	const deleteButton = document.createElement("button");
	deleteButton.className = "delete";
	const deleteImage = document.createElement("img");
	deleteImage.src = "/assets/delete.svg";
	deleteImage.className = "delete-button";
	deleteButton.appendChild(deleteImage);
	deleteDiv.appendChild(deleteButton);
	
	button_wrapper.appendChild(deleteDiv);
	card_wrapper.appendChild(name_wrapper);
	card_wrapper.appendChild(button_wrapper);
	cards.appendChild(card_wrapper);
}
if (
	window.location.pathname === "/" ||
	window.location.pathname === "/index.html"
) {
	let editable_boards; // All metadata for boards that are editable/playable
	let playable_boards; // All metadata for boards that are only playable
	try {
		editable_boards = await invoke("get_bingo_projects");
		playable_boards = await invoke("get_bingo_games");
	} catch (error) {
		editable_boards = [];
		playable_boards = [];
	}
	if (editable_boards.length === 0 && playable_boards.length === 0) {
		const create_board_button = document.createElement("button");
		create_board_button.id = "create-board";
		create_board_button.textContent = "Get Started";
		create_board_button.addEventListener("click", () => {
			sessionStorage.removeItem("path");
			window.location.href = "editable-board/editable-board.html";
		});
		cards.appendChild(create_board_button);
	} else {
		const create_board_button = document.createElement("button");
		create_board_button.id = "create-board";
		create_board_button.textContent = "Create Board";
		create_board_button.addEventListener("click", () => {
			sessionStorage.removeItem("path");
			window.location.href = "editable-board/editable-board.html";
		});
		cards.appendChild(create_board_button);
		
		// add header for Projects
		const project_header = document.createElement("h2");
		project_header.textContent = "Projects";
		project_header.className = "name-wrapper";
		const pLine = document.createElement("hr");
		cards.appendChild(project_header);
		cards.appendChild(pLine);
		
		if (editable_boards != undefined) {
			for (const elem of editable_boards) {
				const [items, path] = elem;
				draw_card(items.title, items.city, path, true);
			}
		}

		// add header for Games
		const game_header = document.createElement("h2");
		game_header.textContent = "Games";
		game_header.className = "name-wrapper";
		const gLine = document.createElement("hr");
		cards.appendChild(game_header);
		cards.appendChild(gLine);
		
		if (playable_boards != undefined) {
			for (const elem of playable_boards) {
				const [items, path] = elem;
				const internalBoard = items.board;
				draw_card(internalBoard.title, internalBoard.city, path, false);
			}
		}
	}
	document.addEventListener("click", (e) => {
		const button = e.target; // Gets either the edit/export buttons
		const button_wrapper = e.target.closest(".button-wrapper");
		if (!button_wrapper) return;
		else {
			sessionStorage.setItem("path", button_wrapper.id);
			if (button.className === "edit-button" || button.className === "edit") {
				window.location.href = "./editable-board/editable-board.html";
			} else if (button.className === "export-button" || button.className === "export") {
				invoke("quick_export", { projPath: sessionStorage.getItem("path") });
				window.location.reload();
			} else if (button.className === "play-button" || button.className === "play") {
				window.location.href = "./play-board/play-board.html";
			} else if (button.className === "delete-button" || button.className === "delete" || button.className === "delete-wrapper") {
				invoke("delete", { path: sessionStorage.getItem("path") });
				window.location.reload();
			}
		}
	});
}
