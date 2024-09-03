// import { initSync, renderCell, renderSchema, SchemaOptions } from './lib/pkg/scada_lib.js';

function setCellAttribute(cell, name, value) {
	//cell.value = new NamedNodeMap();
	cell.setAttribute(name, value);
}

function loadScadaModel(editor, xmlStr) {
	const node = mxUtils.parseXml(xmlStr).documentElement;
	if (!!node) {
		var dec = new mxCodec(node.ownerDocument);
	
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

			let cell0 = editor.graph.getModel().getCell("0");
			if (!!cell0.value && typeof cell0.value !== 'string') {
				// console.log("cell0", cell0.value.outerHTML);
				return cell0.value.outerHTML;
			}
			return undefined;

		}	
	}
}

/**
 * Sample plugin.
 */
Draw.loadPlugin(async function(ui) {
	const {initSync, renderCell, renderSchema, SchemaOptions} = await import('./lib/pkg/scada_lib.js');

	async function initWasm() {
		await fetch('plugins/scada/lib/pkg/scada_lib_bg.wasm')
			.then(r => r.arrayBuffer())
			.then(o => {
				initSync(o);
			});				
	}

	var div = document.createElement('div');
	div.setAttribute("id", "container");
	div.style.background = Editor.isDarkMode() ? Editor.darkColor : '#ffffff';
	div.style.border = '1px solid gray';
	div.style.opacity = '0.8';
	div.style.padding = '10px';
	div.style.paddingTop = '0px';
	div.style.width = '20%';
	// div.innerHTML = '<p><i>' + mxResources.get('nothingIsSelected') + '</i></p>';

	var graph = ui.editor.graph;

	if (!ui.editor.isChromelessView())
	{
		div.style.boxSizing = 'border-box';
		div.style.minHeight = '100%';
		div.style.width = '100%';

		var iiw = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
		
		var dataWindow = new mxWindow('SCADA', div, iiw - 320, 60, 240, 400, true, true);
		dataWindow.destroyOnClose = false;
		dataWindow.setMaximizable(true);
		dataWindow.setResizable(true);
		dataWindow.setScrollable(true);
		dataWindow.setClosable(true);
		dataWindow.contentWrapper.style.overflowY = 'scroll';

		// Adds resource for action
		mxResources.parse('scada=SCADA');

		// Adds action
		ui.actions.addAction('scada...', function()
		{
			dataWindow.setVisible(!dataWindow.isVisible());
		});
		
		var menu = ui.menus.get('extras');
		var oldFunct = menu.funct;
		
		menu.funct = function(menu, parent)
		{
			oldFunct.apply(this, arguments);
			ui.menus.addMenuItems(menu, ['-', 'scada'], parent);
		};
	}
	else
	{
		div.style.position = 'absolute';
		div.style.minWidth = '200px';
		div.style.top = '40px';
		div.style.right = '20px';

		document.body.appendChild(div);
	}
	
	// Highlights current cell
	const highlight = new mxCellHighlight(graph, '#00ff00', 8);
	const ignored = ['label', 'tooltip', 'placeholders'];

	// register_conteiner(ui.editor, div);	// for wasm app
	
	// init wasm application
	// const app = new AppApi(ui.editor, div);

	function writeConsole(evt)
	{
		var result = graph.getDataForCells(graph.getSelectionCells());

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
	function cellClicked(cell, modelChanged)
	{
		// Gets the selection cell
		if (cell == null)
		{
			highlight.highlight(null);
			// app.cell_clicked(null);
			// renderSchema(div, new SchemaOptions("http://zheleschikovav.keenetic.pro:18764/v1/configurator"));
			renderSchema(mxUtils, ui.editor, div, new SchemaOptions("http://localhost:8091/api/v1"));
			console.log("js renderSchema");

			// let snapshot = ui.getDiagramSnapshot();
			// let model = mxUtils.getPrettyXml(snapshot.node);
			// console.log("model", model);

		}
		else
		{
			if (ui.editor.isChromelessView())
			{
				highlight.highlight(graph.view.getState(cell));
			}
	
			// console.log("compare prev", prevcell===cell);

			if (modelChanged) {
				console.log("model changed", cell);
				// app.cell_updated(cell);
			} else {
				//let doc = mxUtils.parseXml("<scada><som-data p='test' as='data'/></scada>").documentElement;
				//cell.setValue(doc);

				//console.log("selection changed", cell.value);
				// app.cell_clicked(cell);

				renderCell(div, cell);
				// prevcell = cell;
			}

			// var attrs = (cell.value != null) ? cell.value.attributes : null;
			// if (attrs != null)
			// {
			// 	var label = Graph.sanitizeHtml(graph.getLabel(cell));
				
			// 	if (label != null && label.length > 0)
			// 	{
			// 		div.innerHTML = '<h1>' + label + '</h1>';
			// 	}
			// 	else
			// 	{
			// 		div.innerText = '';
			// 	}
				
			// 	for (var i = 0; i < attrs.length; i++)
			// 	{
			// 		if (mxUtils.indexOf(ignored, attrs[i].nodeName) < 0 &&
			// 			attrs[i].nodeValue.length > 0)
			// 		{
			// 			// TODO: Add click handler on h2 to output data
			// 			var h2 = document.createElement('h2');
			// 			mxUtils.write(h2, attrs[i].nodeName);
			// 			div.appendChild(h2);
			// 			var p = document.createElement('p');
			// 			mxUtils.write(p, attrs[i].nodeValue);
			// 			div.appendChild(p);
			// 		}
			// 	}

			// 	// set_cell(div, cell);
			// }
			// else
			// {
			// 	var label = graph.convertValueToString(cell);
				
			// 	if (label != '')
			// 	{
			// 		div.innerHTML = '<h1>' + Graph.sanitizeHtml(label) + '</h1>';
			// 	}
			// 	else
			// 	{
			// 		div.innerHTML = '<p><i>No data</i></p>';
			// 	}
			// }

			// if (!ui.editor.isChromelessView())
			// {
			// 	var button = document.createElement('button');
			// 	button.setAttribute('title', 'Click or Shift+Click to write data for all selected cells to the browser console');
			// 	button.style['float'] = 'none';
			// 	mxUtils.write(button, 'Write to Console');

			// 	mxEvent.addListener(button, 'click', function(evt)
			// 	{
			// 		writeConsole(evt);
			// 	});

			// 	div.appendChild(button);
			// }
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

	// =======================================
	// init rust wasm
	await initWasm();


});