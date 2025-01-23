let url = 'http://localhost:8000/';
let url_mock = 'http://localhost:8000/mock/';

interface Table {
    id: number;
    latitude: number;
    longitude: number;
}
export async function getTables(): Promise<Table[]> {
    const response = await fetch(url + 'stations');
    return await response.json();
}

export function getDetails() {
    fetch(url + 'detail_stations/')
        .then(res => res.json())
        .then(data => {
            console.log(data)
        });
}

export function getDetailsById(id: number) {
    fetch(url + 'detail_stations/' + id)
        .then(res => res.json())
        .then(data => {
            console.log(data)
        });
}