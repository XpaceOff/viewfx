<script>
    export let tittle;
    export let value;
    export let min = 10.0;
   
    function validate(e){
        let userValue = e.target.value;
        userValue = parseFloat(userValue.replace(/[^\d.]/g, ''));

        if (isNaN(userValue) || userValue === null || userValue === undefined) userValue = min;
        if (userValue < min) userValue = min;

        value = userValue;
    }
</script>

<div class="flex flex-row w-full hover:bg-zinc-600 px-2 py-1 rounded-lg text-zinc-500 hover:text-zinc-300 items-center">
    <div class="flex w-8/12">
        {tittle}
    </div>
    <!-- <div class="flex w-4/12 ">
        {value}
    </div> -->
    <input 
        class=" bg-zinc-900 rounded-md border-0 px-2"
        value={value}
        on:change={validate}
    />
</div>