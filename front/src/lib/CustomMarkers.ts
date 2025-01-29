import {getDetailsById, getPredict} from "$lib/rust_api";
import L from "leaflet";
import {date} from '$lib/store';

/**
 * Custom icon class to change the color of the icon
 *
 * @param color
 */
class CustomIcon extends L.Icon {
    constructor(color: string = 'black') {
        // Do some SVG manipulation to change the color of the icon
        const svgContent = `
        <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 16 16" fill="none">
            <path fill-rule="evenodd" 
                    clip-rule="evenodd" 
                    d="M3.37892 10.2236L8 16L12.6211 10.2236C13.5137 9.10788 14 7.72154 14 6.29266V6C14 2.68629 
                    11.3137 0 8 0C4.68629 0 2 2.68629 2 6V6.29266C2 7.72154 2.4863 9.10788 3.37892 10.2236ZM8 
                    8C9.10457 8 10 7.10457 10 6C10 4.89543 9.10457 4 8 4C6.89543 4 6 4.89543 6 6C6 7.10457 6.89543 8 8 8Z" 
                    fill="${color}" stroke="black" stroke-width="1"/>
        </svg>`;
        const svgBlob = new Blob([svgContent], {type: 'image/svg+xml'});
        const svgUrl = URL.createObjectURL(svgBlob);

        super({
            iconUrl: svgUrl,
            iconSize: [20, 20],
            iconAnchor: [20, 40],
            popupAnchor: [0, -40]
        });
    }
}

/**
 * CustomMarkers class to handle the markers
 *
 * @param id
 * @param latitude
 * @param longitude
 */
export class CustomMarkers {
    id: number;
    marker: L.Marker;
    station_name: string = "";
    prediction_empty_slots: string = "";
    prediction_available_bike: string = "";

    /**
     * Constructor for the CustomMarkers class
     *
     * @param id
     * @param latitude
     * @param longitude
     */
    constructor(id: number, latitude: number, longitude: number) {
        this.id = id;
        this.marker = L.marker([latitude, longitude], {icon: new CustomIcon()});
        this.marker.bindPopup(""); // Bind an empty popup initially

        this.marker.addEventListener('click', async () => {
            //get station name
            await this.refreshStationName();
            //get predictions
            await this.refreshPrediction();
            //display popup on click of markers
            this.marker.setPopupContent("<h3>" + this.station_name + "</h3><p>Velo'v disponibles : " + this.prediction_available_bike + "</p> <p>Bornes disponibles : " + this.prediction_empty_slots + "</p>");
            this.marker.openPopup();
        });

        //display tooltip on hover of markers
        this.marker.addEventListener('mouseover', async () => {
            if (this.station_name == "") {
                await this.refreshStationName();
            }
            this.marker.bindTooltip(this.station_name);
        });
    }

    /**
     * Get the marker
     *
     * @returns the marker
     */
    getMarker() {
        return this.marker;
    }

    /**
     * Change the color of the marker
     *
     * @param color
     */
    changeColor(color: string) {
        this.marker.setIcon(new CustomIcon(color));
    }

    /**
     * Get the id of the marker
     *
     * @returns the id
     */
    getId() {
        return this.id;
    }

    /**
     * Refresh the station name by calling the API
     */
    async refreshStationName() {
        // We get the details for the selected station
        const station_data = await getDetailsById(this.id);

        if (station_data.name != "") {
            this.station_name = station_data.name
        } else {
            this.station_name = "no available data"
        }
    }

    /**
     * Refresh the prediction by calling the API
     */
    async refreshPrediction() {
        // We get the selected date from the store
        let selected_date: Date = new Date(0, 1, 1); // Initialised to avoid confusion
        date.subscribe(value => selected_date = value)();

        if (selected_date >= new Date()) {
            // We do some date manipulation to get the next hour with minutes and seconds set to 0
            selected_date.setHours(selected_date.getHours() + 1);
            selected_date.setMinutes(0);
            selected_date.setSeconds(0);

            let dateStr = selected_date.toLocaleString('sv-SE', {
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit'
            }).replace(' ', 'T');

            // We get the prediction for the selected date
            let predicted_data = await getPredict(this.id, dateStr);

            if (predicted_data == null) {
                this.prediction_available_bike = 'indisponible';
                this.prediction_empty_slots = 'insdisponible';
            } else {
                this.prediction_available_bike = String(predicted_data?.available_bikes);
                this.prediction_empty_slots = String(predicted_data?.free_stands);
            }

            console.log(predicted_data);
        } else {
            console.log('Date is in the past');

            this.prediction_available_bike = 'indisponible';
            this.prediction_empty_slots = 'insdisponible';
        }
    }

}