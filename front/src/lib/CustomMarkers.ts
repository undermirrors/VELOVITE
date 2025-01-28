import {getDetailsById} from "$lib/rust_api";
import L from "leaflet";

class Icon extends L.Icon {
    constructor() {
        super({iconUrl: 'marker_icon.svg'});
    }
}

export class CustomMarkers {
    id: number;
    marker: L.Marker;

    constructor(id: number, latitude: number, longitude: number) {
        this.id = id;
        const blackIcon = new Icon();
        this.marker = L.marker([latitude, longitude], {icon: blackIcon}).addEventListener('click', async () => {
            let advanced_data = await getDetailsById(this.id);
            console.log(advanced_data);
        });
    }

    getMarker() {
        return this.marker;
    }

    getId() {
        return this.id;
    }
}