
function mxCssLink(href)
{
	var s = document.createElement('link');
	s.setAttribute('rel', 'stylesheet');
	s.setAttribute('href', href);

	var t = document.getElementsByTagName('link')[0];
	if (t != null)
	{
		t.parentNode.insertBefore(s, t);
	}
};

function setCellAttribute(cell, name, value) {
	//cell.value = new NamedNodeMap();
	cell.setAttribute(name, value);
}

function loadDFlowModel(editor, xmlStr) {
	const node = mxUtils.parseXml(xmlStr).documentElement;
	if (node) {
		let dec = new mxCodec(node.ownerDocument);

		if (node.nodeName == 'mxGraphModel')
		{
			editor.graph.model.beginUpdate();
			try
			{
				editor.graph.model.clear();
				editor.graph.view.scale = 1;
				editor.readGraphState(node);
				editor.updateGraphComponents();
				dec.decode(node, editor.graph.getModel());
			}
			finally
			{
				editor.graph.model.endUpdate();
			}

			editor.fireEvent(new mxEventObject('resetGraphView'));
		}
	}
}

function getCell0(editor) {
	return editor.graph.getModel().getCell("0");
}

function setCell0Value(editor, value) {
	const node = mxUtils.parseXml(value).documentElement;
	if (node) {
		editor.graph.model.beginUpdate();
		try
		{
			let cell0 = editor.graph.model.getCell("0");
			cell0.value = node;
		}
		finally
		{
			editor.graph.model.endUpdate();
		}
	}
}

function getPrettyXml(element) {
	return mxUtils.getPrettyXml(element);
}

// двигает модель в (0,0) угол
function clipedModelBox(modelStr) {
	let node = mxUtils.parseXml(modelStr).documentElement;
	if (!!node && node.nodeName === 'mxGraphModel') {
		let container = document.createElement("div");
		let graph2 = new mxGraph(container);
		let codec = new mxCodec(node);

		graph2.model.beginUpdate();
		try
		{
			graph2.model.clear();
			codec.decode(node, graph2.getModel());
		}
		finally
		{
			graph2.model.endUpdate();
		}

		graph2.model.beginUpdate();
		try {
			let cells = graph2.model.cells;
			let widgetCells = Object.entries(cells).map(( [k, v] ) => v);

			// make all cells uneditable
			// movable=0;resizable=0;rotatable=0;cloneable=0;deletable=0
			let mxCells = widgetCells.filter((o) => o.id != "0" && o.id != "1");
			graph2.setCellStyles("movable", 0, mxCells);
			graph2.setCellStyles("resizable", 0, mxCells);
			graph2.setCellStyles("rotatable", 0, mxCells);
			graph2.setCellStyles("cloneable", 0, mxCells);
			graph2.setCellStyles("deletable", 0, mxCells);

			let box = graph2.getBoundingBox(widgetCells);
			let x = box.x;
			let y = box.y;
			widgetCells.forEach(c => {
				let geom = c.getGeometry();
				if (!!geom) {
					geom.x -= x;
					geom.y -= y;
				}
			});
		}
		finally {
			graph2.model.endUpdate();
		}

		let encoder = new mxCodec();
		let result = encoder.encode(graph2.model);
		let res =  mxUtils.getXml(result, '');
		container.remove();
		return res;
	}

	return modelStr;
}

function setWidgetModel(editor, cellP, modelStr) {
	let node = mxUtils.parseXml(modelStr).documentElement;
	if (node && node.nodeName === 'mxGraphModel') {
		let container = document.createElement("div");
		let graph2 = new mxGraph(container);
		let codec = new mxCodec(node);

		let widgetCells = [];

		graph2.model.beginUpdate();
		try
		{
			graph2.model.clear();
			// graph2.view.scale = 0.5;
			codec.decode(node, graph2.getModel());
		}
		finally
		{
			graph2.model.endUpdate();
		}

		graph2.model.beginUpdate();
		try {
			let cells = graph2.model.cells;
			widgetCells = Object.entries(cells).map(( [k, v] ) => v);

			let box = graph2.getBoundingBox(widgetCells);
			let pgeom = cellP.getGeometry();
			pgeom.width = box.width;
			pgeom.height = box.height;

			let x = box.x;
			let y = box.y;
			let idp = cellP.getId();
			widgetCells.forEach(c => {
				let geom = c.getGeometry();
				if (!!geom) {
					geom.x -= x;
					geom.y -= y;
				}
				let newId =idp + '#' + c.getId();
				c.setId(newId);
			});
		}
		finally {
			graph2.model.endUpdate();
		}

		// merge into cellP
		editor.graph.model.beginUpdate();
		try {
			// let glyph = cellP.remove(0);  // remove glyph cell
			let childs = cellP.children;
			editor.graph.setCellStyles("deletable", 1, childs);
			// childs.forEach((_, i) => cellP.remove(i))
			editor.graph.removeCells(childs);
			// childs.forEach(o => editor.graph.removeStateForCell(o));

			let cells = graph2.model.cells;
			editor.graph.model.mergeChildren(cells["1"], cellP, false);
		}
		finally {
			editor.graph.model.endUpdate();
			container.remove();
		}

		editor.graph.refresh(cellP);
	}
}

function getGraphSvg(editor) {
		/*
		Graph.prototype.getSvg = function(
		background,  --optional collor
		scale, 	--optional
		border,  --optional
		nocrop,  --optional
		crisp, --optional
		ignoreSelection, --optional bool
		showText, --optional bool
		imgExport,  --null
		linkTarget,
		hasShadow,
		incExtFonts,
		theme,
		exportType,
		cells,
		noCssClass,
		disableLinks --true
		)
		*/

		let svg = editor.graph.getSvg();
		return mxUtils.getXml(svg, '');
}

async function getPaletteData(apiUrl) {
	try {
	  const response = await fetch(`${apiUrl}/widget-group/all`);
	  if (!response.ok) {
		throw new Error(`Response status: ${response.status}`);
	  }

	  const json = await response.json();
	  return json;
	} catch (error) {
	  console.error(error.message);
	}
}

// пробуем отределить тип модели (diargam/widget)
function getModelType(model) {
	let cell = model.cells["0"];
	if (typeof cell.value === 'object') {
		let name = cell.value.firstChild.tagName;
		return name === "widget" || name === "diagram";
	}
	return false;
}

function createDiagramWindow(title) {
	let container = document.createElement('div');
	container.setAttribute("id", "diagram-container");
	container.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	container.style.border = '1px solid gray';
	container.style.opacity = '0.8';
	// container.style.padding = '10px';
	container.style.paddingTop = '0px';
	container.style.width = '20%';
	container.style.boxSizing = 'border-box';
	container.style.minHeight = '100%';
	container.style.width = '100%';

	let iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;

	const wind = new mxWindow(title, container, iiw - 520, 60, 300, 500, true, true);
	wind.destroyOnClose = false;
	wind.setMaximizable(true);
	wind.setResizable(true);
	wind.setScrollable(true);
	wind.setClosable(true);
	wind.contentWrapper.style.overflowY = 'scroll';

	return [wind, container];
}

function isDFlowCell(cell)
{
	if (!!cell && !!cell.value && typeof cell.value !== 'string')
	{
		return cell.value.tagName === "d-flow";
	}
	return false;
};


function createCellindow() {
	let container = document.createElement('div');
	container.setAttribute("id", "info-container");
	container.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	container.style.border = '1px solid gray';
	container.style.opacity = '0.8';
	container.style.paddingTop = '0px';
	container.style.width = '20%';
	container.style.boxSizing = 'border-box';
	container.style.minHeight = '100%';
	container.style.width = '100%';

	let iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;

	const wind = new mxWindow("Настройки элемента", container, iiw - 520, 60, 300, 500, true, true);
	wind.destroyOnClose = true;
	wind.setMaximizable(true);
	wind.setResizable(true);
	wind.setScrollable(true);
	wind.setClosable(true);
	wind.contentWrapper.style.overflowY = 'scroll';

	return [wind, container];
}

const API_URL = "http://localhost:8091/api/v1";
let diagramDataWindow = null;
let schemaRootContainer = null;
let cellDataWindow = null;
let cellRootContainer = null;

// --------------------------------
function destroyDiagramWind() {
	if (diagramDataWindow) {
		let root = diagramDataWindow.getElement();
		if (root) {
			root.remove();
		}
		diagramDataWindow.destroy;
		diagramDataWindow = null;
		schemaRootContainer.remove();
	}
}

// --------------------------------
function destroyCellWind() {
	if (cellDataWindow) {
		let root = cellDataWindow.getElement();
		if (root) {
			root.remove();
		}
		cellDataWindow.destroy;
		cellDataWindow = null;
		cellRootContainer.remove();
	}
}

// --------------------------------
function recreateWidgetModelInfo(editor, modelStr, renderFun) {
	console.log("recreateWidgetModelInfo");
	destroyDiagramWind();

	const [wind, rootContainer] = createDiagramWindow("Настройки виджета");
	diagramDataWindow = wind;
	schemaRootContainer = rootContainer;

	loadDFlowModel(editor, modelStr)
	renderFun(schemaRootContainer);

	diagramDataWindow.show();
}

// --------------------------------
function recreateDiagramModelInfo(editor, modelStr, renderFun) {
	console.log("recreateDiagramModelInfo");
	destroyDiagramWind();

	const [wind, rootContainer] = createDiagramWindow("Настройки диаграммы");
	diagramDataWindow = wind;
	schemaRootContainer = rootContainer;

	loadDFlowModel(editor, modelStr)
	renderFun(schemaRootContainer);

	diagramDataWindow.show();
}

// --------------------------------
function recreateCellInfo(renderFun) {
	console.log("recreateCellInfo");
	destroyCellWind();

	const [wind, rootContainer] = createCellindow();
	cellDataWindow = wind;
	cellRootContainer = rootContainer;

	renderFun(cellRootContainer);

	cellDataWindow.show();
}



/**
 * Sample plugin.
 */
Draw.loadPlugin(async function(ui) {
	const {initSync, renderCellInfo, recreateModelMeta, openDialog, SchemaOptions } = await import('./lib/pkg/dflow_lib.js');

	async function initWasm() {
		await fetch('plugins/dflow/lib/pkg/dflow_lib_bg.wasm')
			.then(r => r.arrayBuffer())
			.then(o => {
				initSync(o);
			});
	}

	// ============= CSS =====================
	mxCssLink("plugins/dflow/css/styles.css");

	let graph = ui.editor.graph;
	// ============= windows ==================

	// Highlights current cell
	const highlight = new mxCellHighlight(graph, '#00ff00', 2);

	function writeConsole(evt)
	{
		let result = graph.getDataForCells(graph.getSelectionCells());

		if (mxEvent.isShiftDown(evt))
		{
			console.log(JSON.stringify(result, null, '  '));
		}
		else
		{
			console.log(result);
		}
	};

	// let prevcell = undefined;
	/**
	 * Updates the properties panel
	 */
	let isRendered = false;
	function cellClicked(cell, modelChanged)
	{
		// Gets the selection cell
		if (cell == null)
		{
			highlight.highlight(null);
			if (!isRendered) {
				// console.log("CALL renderSchema");
				// renderSchema(mxUtils, ui.editor, schemaDiv, getAppOptions());
				isRendered = true;
			}
		}
		else
		{
			if (ui.editor.isChromelessView())
			{
				highlight.highlight(graph.view.getState(cell));
			}

			if (modelChanged) {
				console.log("model changed", cell);
				// app.cell_updated(cell);
			}
		}
	};

	if (!ui.editor.isChromelessView())
	{
		graph.selectionModel.addListener(mxEvent.CHANGE, function(sender, evt)
		{
			cellClicked(graph.getSelectionCell(), false);
		});

		graph.model.addListener(mxEvent.CHANGE, function(sender, evt)
		{
			cellClicked(graph.getSelectionCell(), true);
		});
	}
	else
	{
		graph.click = function(me)
		{
			// Async required to enable hyperlinks in labels
			window.setTimeout(function()
			{
				cellClicked(me.getCell());
			}, 0);
		};
	}

	// ================== SIDEBAR ===================
	// Adds sidebar entries
	let sb = ui.sidebar;
	async function addPalette() {

		const widgetGroups = await getPaletteData(API_URL);
		console.log(widgetGroups);

		sb.addPalette('dflow', 'DFlow items', true, mxUtils.bind(sb, function(content) {
			widgetGroups.forEach(group => {
				let container = new mxCell('', new mxGeometry(0, 0, 100, 100), 'container=1;collapsible=0;connectable=0;strokeColor=none;');
				container.vertex = true;

				let value = mxUtils.parseXml(group.model).documentElement;
				value.setAttribute('label', container.value || '');
				container.setValue(value);

				let glyph = new mxCell('', new mxGeometry(3, 3, 94, 94),
					`shape=image;imageAspect=0;aspect=fixed;verticalLabelPosition=bottom;verticalAlign=top;image=${group.image};movable=0;rotatable=0;cloneable=0;connectable=0;resizable=0;allowArrows=0;`
				);
				glyph.vertex = true;
				container.insert(glyph);
				content.appendChild(sb.createVertexTemplateFromCells([container], 100, 40, group.name));
			});
		}));
	}
	await addPalette();

	// Handles reload of sidebar after dark mode change
	let init = sb.init;
	sb.init = async function()
	{
		init.apply(this, arguments);
		await addPalette();
	};


	// ================ MENUS =================
	// Adds menu
	mxResources.parse('createDiagram=Новая диаграмма');
	mxResources.parse('createWidget=Новый виджет');
	mxResources.parse('openItem=Открыть...');
	mxResources.parse('dflow=Настройки полотна');
	mxResources.parse('dflowData=Настройки элемента');

	// --------------
    ui.actions.addAction('createDiagram', function()
    {
		recreateDiagramModelInfo(
			ui.editor,
			'<mxGraphModel dx="1173" dy="736" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0"><root><object label="" id="0"><diagram /><mxCell /></object><mxCell id="1" parent="0" /></root></mxGraphModel>',
			(schemaRootContainer) => recreateModelMeta("diagram", ui.editor, mxUtils, schemaRootContainer, getAppOptions()),
		)

		if (getModelType(graph.model)) {
			dflowAction.enabled = true;
		}
    });

	// --------------
    ui.actions.addAction('createWidget', function()
    {
		recreateWidgetModelInfo(
			ui.editor,
			'<mxGraphModel dx="1173" dy="736" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="850" pageHeight="1100" math="0" shadow="0"><root><object label="" id="0"><widget object-type=""/><mxCell /></object><mxCell id="1" parent="0" /></root></mxGraphModel>',
			(schemaRootContainer) => recreateModelMeta("widget", ui.editor, mxUtils, schemaRootContainer, getAppOptions()),
		)

		if (getModelType(graph.model)) {
			dflowAction.enabled = true;
		}
    });

	// --------------
	const openOpenDialogAction = ui.actions.addAction('openItem', function()
    {
		ui.showDialog(new DFlowItemsDialog(ui).container, 500, 400, true, false);
    });
	openOpenDialogAction.enabled = true;

	// --------------
	const dflowAction = ui.actions.addAction('dflow', function()
	{
		if (getModelType(graph.model)) {
			diagramDataWindow.setVisible(!diagramDataWindow.isVisible());
		} else {
			diagramDataWindow.hide();
		}
	});
	dflowAction.enabled = false;

	// --------------
	const dflowDataAction = ui.actions.addAction('dflowData', function()
	{
		cellDataWindow.setVisible(!cellDataWindow.isVisible());
	});
	dflowDataAction.enabled = false;

	// --------------
	ui.menus.put('dflow', new Menu(function(menu, parent)
	{
		ui.menus.addMenuItems(menu, ['createDiagram', 'createWidget', '-', 'openItem', '-', 'dflow', 'dflowData']);
	}));

    if (ui.menubar != null)
    {
		var menu = ui.menubar.addMenu('DFlow', ui.menus.get('dflow').funct);
		menu.parentNode.insertBefore(menu, menu.previousSibling.previousSibling.previousSibling);
    }

	/**
	 * Updates the DFlow data panel
	 */
	// let prevCellId = undefined;
	function dflowCellClicked(cell)
	{
		// Gets the selection cell
		if (cell != null && isDFlowCell(cell))
		{
			recreateCellInfo(
				(schemaRootContainer) => renderCellInfo(cell, ui.editor, mxUtils, schemaRootContainer, getAppOptions()),
			)

			cellDataWindow.setVisible(true);

			// highlight selected cell
			highlight.highlight(graph.view.getState(cell));
		}
		else {
			highlight.highlight(null);
			destroyCellWind();
		}

	}

	if (!ui.editor.isChromelessView())
	{
		graph.selectionModel.addListener(mxEvent.CHANGE, function(sender, evt)
		{
			dflowCellClicked(graph.getSelectionCell());
		});
	}

	// Adds resources for actions
	mxResources.parse('dflowItem=DFlow item');

	// Adds actions
	ui.actions.addAction('dflowItem', function()
	{
		if (graph.isEnabled() && graph.getSelectionCount() == 1)
		{
			let cell = graph.getSelectionCell();
			if (!isDFlowCell(cell)) {
				let value = mxUtils.parseXml("<d-flow><undefiend/></d-flow>").documentElement;
				value.setAttribute('label', cell.value || '');
				cell.setValue(value);
				dflowCellClicked(cell);
			}
		}
	}, null, null, 'Alt+Shift+W');

	// -----------------------------------------------------------------
	let uiCreatePopupMenu = ui.menus.createPopupMenu;
	ui.menus.createPopupMenu = function(menu, cell, evt)
	{
		uiCreatePopupMenu.apply(this, arguments);

		menu.addSeparator();
		// let cell = graph.getSelectionCell();
		if (!isDFlowCell(cell)) {
			this.addMenuItems(menu, ['dflowItem'], null, evt);
		}

		if (isDFlowCell(cell) && graph.getSelectionCount() == 1)
		{
			this.addMenuItems(menu, ['dflowData'], null, evt);

			// if (sib != null && sib.length > 0)
			// {
			// 	this.addMenuItems(menu, ['selectChildren', 'selectSubtree'], null, evt);
			// }

			// menu.addSeparator();

			// if (cell.getAttribute('treeRoot') != '1')
			// {
			// 	this.addMenuItems(menu, ['selectSiblings', 'selectParent'], null, evt);
			// }
		}
	};
	// =======================================
	class DFlowItemsDialog {
		constructor(editorUi) {
			var div = document.createElement('div');
			openDialog(mxUtils, editorUi, editorUi.editor, div, getAppOptions());
			this.container = div;
		}
	}

	// ============== WASM ===================
	// init rust wasm
	await initWasm();
	// здесь натройки пдагина
	let getAppOptions = function() {return new SchemaOptions(API_URL); }

	// cellDataWindow = newCellWindow(divDFlowCellData);
	// initCellRender(ui.editor, mxUtils, divDFlowCellData, getAppOptions());

});
