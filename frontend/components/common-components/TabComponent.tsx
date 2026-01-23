"use client"

type TabComponentProps = {
    tab: number, 
    setTab: (index: number) => void, 
    index: number, 
    label: string
}


export function TabComponent({tab, setTab, index, label} : TabComponentProps) {
    return <div style={{color: tab === index ? "#F59E0B" : "white", display: "flex", flexDirection: "column", width: "100px", gap: "5px"}} 
        onClick={() => {
            setTab(index);
    }}>
        <div className="flex justify-center cursor-pointer hover:text-[#F59E0B] transition delay-100">
            {label}
        </div>
        {
            tab === index 
            &&
            <div style={{width: "100%", height: "2px", backgroundColor: "#F59E0B", alignItems: "center", justifyContent: "center"}}/>
        }
        
    </div>
}