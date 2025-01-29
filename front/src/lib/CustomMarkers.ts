import {getDetailsById, getPredict} from "$lib/rust_api";
import L from "leaflet";
import {date} from '$lib/store';

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

export class CustomMarkers {
    id: number;
    marker: L.Marker;

    constructor(id: number, latitude: number, longitude: number) {
        this.id = id;
        this.marker = L.marker([latitude, longitude], {icon: new CustomIcon()}).addEventListener('click', async () => {
            // We get the details for the selected station
            let advanced_data = await getDetailsById(this.id);

            // We get the selected date from the store
            let selected_date: string = '';
            date.subscribe(value => selected_date = value)();

            console.log('Marker clicked');
            console.log(advanced_data);
            console.log('value :' + selected_date);
            if (selected_date >= new Date().toISOString()) {
                // We do some date manipulation to get the next hour with minutes and seconds set to 0
                let date_id = new Date(
                    new Date(selected_date).setHours(
                        Number(selected_date.split('T')[1].split(':')[0]) + 1, 0, 0, 0)
                ).toISOString();
                console.log(date_id)
                date_id = date_id.replaceAll(".000Z", "Z");

                // We get the prediction for the selected date
                let predicted_data = await getPredict(this.id, date_id);

                console.log(date_id)
                console.log(predicted_data);

            } else {
                console.log('Date is in the past');
            }
        });
        this.marker.bindPopup("<h3>Nom de la station</h3><p>10%</p>");
        this.marker.bindTooltip("station");
    }


    getMarker() {
        return this.marker;
    }

    changeColor(color: string) {
        this.marker.setIcon(new CustomIcon(color));
    }

    getId() {
        return this.id;
    }
}