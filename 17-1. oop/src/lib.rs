
// 구조체가 pub 키워드에 의해 다른 코드가 사용할 수 있게 되었지만 내부 항목들은 비공개임
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/*
    공개된 add, remove, average만이 인스턴스를 수정하는 유일한 방법
    list, average 필드를 비공개로 두었기 때문에 외부에서 수정 불가능하기 때문에 외부에서 list를 추가하거나 제거할 수 없음
    average는 읽기전용
*/