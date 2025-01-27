<script>
	import LeafletMap from '$lib/LeafletMap.svelte';
    

</script>

<style>
    div {
        height: 100%;
        font-family:Verdana, Tahoma, sans-serif;
        position: relative; /* Nécessaire pour créer un contexte de positionnement */
    }

    .overlay {
        position: absolute;
        border-style:none;
        border-color: grey;
        font-size:medium;
        z-index: 999; /* Plus élevé que la carte */
        background-color: rgba(255, 255, 255, 1); /* Optionnel : un fond pour plus de lisibilité */
        padding: 5px;
        border-radius: 60px;
        height: 60px;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    }

    .overlay-logo {
        top: 20px;
        right: 20px;
        padding:0;
        background: none;
        border-radius: 60px;
    }

    .overlay-meteo {
        top: 20px;
        border-style: none;
        right: 120px;
        height: 60px;
        width: 70px;
        border-radius: 20px;
    }

    .overlay-date {
        padding: 10px;
        border-radius: 20px;
        top: 4%;
        left: 2%;
        height: 20px;
        width: 150px;
    }

    .overlay-adress {
        padding: 10px;
        border-radius: 20px;
        top: 12%;
        left: 2%;
        height: 20px;
        width: 150px;
    }

    .overlay-stations {
        top: 20%;
        left: 2%;
        padding: 10px;
        border-radius: 20px;
        height: 200px;
        width: 150px;
    }


</style>

<div>
    <input class="overlay overlay-adress" type="text" id="adress" name="adress" placeholder="Adresse" />
     
    <input class="overlay overlay-date" type="date" id="date" name="date" placeholder="Date"/>

    <div class="overlay overlay-stations">
        Stations proches
    </div>
    
    <img
    src="/autre-chose.png"
    alt="Meteo" 
    class="overlay overlay-meteo"/>
    
    <img
        src="/dessin.svg"
        alt="Velovite, le site qui vous donne des vélos, vite." 
        class="overlay overlay-logo"/>
        
    

    <LeafletMap />
</div>

